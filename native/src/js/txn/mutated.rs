use neon::prelude::*;

use std::sync::{Arc, Mutex as StdMutex};

use tokio::sync::Mutex;
use tokio::sync::mpsc;

use crate::js::client::JsDgraphClient;
use crate::js::mutation::JsMutation;
use crate::classes::MutatedTxnWrapper;
use crate::tasks::PollTask;
use crate::utils::jsobject_to_hashmap;

declare_types! {
  pub class JsMutatedTxn for MutatedTxnWrapper {
    init(mut ctx) {
      let client = ctx.argument::<JsDgraphClient>(0)?;
      let guard = ctx.lock();
      let client = client.borrow(&guard);

      let (tx, rx) = mpsc::unbounded_channel();

      Ok(MutatedTxnWrapper {
        txn: Arc::new(Mutex::new(Some(client.new_mutated_txn()))),
        response_tx: tx,
        response_rx: Arc::new(StdMutex::new(rx)),
      })
    }

    method discard(mut ctx) {
      let this = ctx.this();
      let guard = ctx.lock();

      let txn_id = this.borrow(&guard).discard();

      Ok(ctx.string(txn_id).upcast())
    }

    method commit(mut ctx) {
      let this = ctx.this();
      let guard = ctx.lock();

      let txn_id = this.borrow(&guard).commit();

      Ok(ctx.string(txn_id).upcast())
    }

    method mutate(mut ctx) {
      let mutation = ctx.argument::<JsMutation>(0)?;

      let this = ctx.this();
      let guard = ctx.lock();

      let mu = mutation.borrow(&guard).clone();

      let txn_id = this.borrow(&guard).mutate(mu.clone());

      Ok(ctx.string(txn_id).upcast())
    }

    method upsert(mut ctx) {
      let query = ctx.argument::<JsString>(0)?.value();
      let mutation = ctx.argument::<JsMutation>(1)?;

      let this = ctx.this();
      let guard = ctx.lock();

      let mu = mutation.borrow(&guard).clone();

      let txn_id = this.borrow(&guard).upsert(query.clone(), mu.clone());

      Ok(ctx.string(txn_id).upcast())
    }

    method upsertWithVars(mut ctx) {
      let query = ctx.argument::<JsString>(0)?.value();

      let vars_obj = ctx.argument::<JsObject>(1)?;
      let vars = jsobject_to_hashmap(&mut ctx, vars_obj).unwrap();

      let mutation = ctx.argument::<JsMutation>(2)?;

      let this = ctx.this();
      let guard = ctx.lock();

      let mu = mutation.borrow(&guard).clone();

      let txn_id = this.borrow(&guard).upsert_with_vars(query.clone(), vars.clone(), mu.clone());

      Ok(ctx.string(txn_id).upcast())
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
