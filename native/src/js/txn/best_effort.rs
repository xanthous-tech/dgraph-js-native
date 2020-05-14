use neon::prelude::*;

use std::sync::mpsc;
use std::sync::{Arc, Mutex};

use crate::js::client::JsDgraphClient;
use crate::classes::BestEffortQueryTxnWrapper;
use crate::tasks::PollTask;
use crate::utils::jsobject_to_hashmap;

declare_types! {
  pub class JsBestEffortTxn for BestEffortQueryTxnWrapper {
    init(mut ctx) {
      let client = ctx.argument::<JsDgraphClient>(0)?;
      let guard = ctx.lock();
      let client = client.borrow(&guard);

      let (tx, rx) = mpsc::channel();

      Ok(BestEffortQueryTxnWrapper {
        txn: Arc::new(Mutex::new(Some(client.new_best_effort_txn()))),
        response_tx: tx,
        response_rx: Arc::new(Mutex::new(rx)),
      })
    }

    method query(mut ctx) {
      let query = ctx.argument::<JsString>(0)?.value();

      let this = ctx.this();
      let guard = ctx.lock();

      let txn_id = this.borrow(&guard).query(query.clone());

      Ok(ctx.string(txn_id).upcast())
    }

    method queryWithVars(mut ctx) {
      let query = ctx.argument::<JsString>(0)?.value();
      let vars_obj = ctx.argument::<JsObject>(1)?;

      let vars = jsobject_to_hashmap(&mut ctx, vars_obj).unwrap();

      let this = ctx.this();
      let guard = ctx.lock();

      let txn_id = this.borrow(&guard).query_with_vars(query.clone(), vars.clone());

      Ok(ctx.string(txn_id).upcast())
    }

    method poll(mut ctx) {
      let cb = ctx.argument::<JsFunction>(0)?;
      let this = ctx.this();
      let guard = ctx.lock();

      let rx = this.borrow(&guard).response_rx.clone();
      Arc::downgrade(&rx);

      let task = PollTask {
        rx,
      };

      task.schedule(cb);

      Ok(ctx.undefined().upcast())
    }
  }
}
