use neon::prelude::*;

use std::sync::{Arc, Mutex};

use crate::js::client::JsDgraphClient;
use crate::classes::ReadOnlyQueryTxnWrapper;
use crate::tasks::{QueryTask, QueryWithVarsTask};
use crate::utils::convert_js_vars_object;

declare_types! {
  pub class JsReadOnlyTxn for ReadOnlyQueryTxnWrapper {
    init(mut ctx) {
      let client = ctx.argument::<JsDgraphClient>(0)?;
      let guard = ctx.lock();
      let client = client.borrow(&guard);

      Ok(ReadOnlyQueryTxnWrapper { txn: Arc::new(Mutex::new(client.new_read_only_txn())) })
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
      let vars = convert_js_vars_object(&mut ctx, vars_obj).unwrap();

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
