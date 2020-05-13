use neon::prelude::*;

use std::sync::{Arc, Mutex};

use crate::js::client::JsDgraphClient;
use crate::classes::BestEffortQueryTxnWrapper;
use crate::tasks::{QueryWithVarsTask, QueryTask};
use crate::utils::jsobject_to_hashmap;

declare_types! {
  pub class JsBestEffortTxn for BestEffortQueryTxnWrapper {
    init(mut ctx) {
      let client = ctx.argument::<JsDgraphClient>(0)?;
      let guard = ctx.lock();
      let client = client.borrow(&guard);

      Ok(BestEffortQueryTxnWrapper { txn: Arc::new(Mutex::new(Some(client.new_best_effort_txn()))) })
    }

    method query(mut ctx) {
      let query = ctx.argument::<JsString>(0)?.value();
      let cb = ctx.argument::<JsFunction>(1)?;

      let this = ctx.this();
      let guard = ctx.lock();

      let txn = this.borrow(&guard).txn.clone();
      Arc::downgrade(&txn);

      let task = QueryTask {
        txn,
        query,
      };

      task.schedule(cb);

      Ok(ctx.undefined().upcast())
    }

    method queryWithVars(mut ctx) {
      let query = ctx.argument::<JsString>(0)?.value();
      let vars_obj = ctx.argument::<JsObject>(1)?;
      let cb = ctx.argument::<JsFunction>(2)?;

      let this = ctx.this();
      let guard = ctx.lock();

      let txn = this.borrow(&guard).txn.clone();
      Arc::downgrade(&txn);
      let vars = jsobject_to_hashmap(&mut ctx, vars_obj).unwrap();

      let task = QueryWithVarsTask {
        txn,
        query,
        vars,
      };

      task.schedule(cb);

      Ok(ctx.undefined().upcast())
    }
  }
}