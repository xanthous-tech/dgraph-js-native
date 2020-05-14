use neon::prelude::*;

use std::sync::{Arc, Mutex};

use crate::js::client::JsDgraphClient;
use crate::classes::ReadOnlyQueryTxnWrapper;
use crate::tasks::ResponseTask;
use crate::utils::jsobject_to_hashmap;

declare_types! {
  pub class JsReadOnlyTxn for ReadOnlyQueryTxnWrapper {
    init(mut ctx) {
      let client = ctx.argument::<JsDgraphClient>(0)?;
      let guard = ctx.lock();
      let client = client.borrow(&guard);

      Ok(ReadOnlyQueryTxnWrapper { txn: Arc::new(Mutex::new(Some(client.new_read_only_txn()))) })
    }

    method query(mut ctx) {
      let query = ctx.argument::<JsString>(0)?.value();
      let cb = ctx.argument::<JsFunction>(1)?;

      let this = ctx.this();
      let guard = ctx.lock();

      let txn_arc_mutex = this.borrow(&guard).txn.clone();
      Arc::downgrade(&txn_arc_mutex);

      smol::run(async {
        use dgraph_tonic::Query;
        use dgraph_tonic::DgraphError;

        let mut mutex_guard = txn_arc_mutex.lock().unwrap();
        let txn = mutex_guard.as_mut();

        let response = match txn {
          Some(t) => t.query(query.clone()).await,
          None => Err(DgraphError::EmptyTxn)
        };

        let task = ResponseTask {
          response: Mutex::new(Some(response)),
        };

        task.schedule(cb);
      });

      Ok(ctx.undefined().upcast())
    }

    method queryWithVars(mut ctx) {
      let query = ctx.argument::<JsString>(0)?.value();
      let vars_obj = ctx.argument::<JsObject>(1)?;
      let cb = ctx.argument::<JsFunction>(2)?;

      let vars = jsobject_to_hashmap(&mut ctx, vars_obj).unwrap();

      let this = ctx.this();
      let guard = ctx.lock();

      let txn_arc_mutex = this.borrow(&guard).txn.clone();
      Arc::downgrade(&txn_arc_mutex);

      smol::run(async {
        use dgraph_tonic::Query;
        use dgraph_tonic::DgraphError;

        let mut mutex_guard = txn_arc_mutex.lock().unwrap();
        let txn = mutex_guard.as_mut();

        let response = match txn {
          Some(t) => t.query_with_vars(query.clone(), vars.clone()).await,
          None => Err(DgraphError::EmptyTxn)
        };

        let task = ResponseTask {
          response: Mutex::new(Some(response)),
        };

        task.schedule(cb);
      });

      Ok(ctx.undefined().upcast())
    }
  }
}
