use neon::prelude::*;

use std::sync::{Arc, Mutex};

use crate::js::client::JsDgraphClient;
use crate::js::mutation::JsMutation;
use crate::classes::MutatedTxnWrapper;
use crate::tasks::{ResponseTask, DgraphCallbackTask};
use crate::utils::jsobject_to_hashmap;

declare_types! {
  pub class JsMutatedTxn for MutatedTxnWrapper {
    init(mut ctx) {
      let client = ctx.argument::<JsDgraphClient>(0)?;
      let guard = ctx.lock();
      let client = client.borrow(&guard);

      Ok(MutatedTxnWrapper { txn: Arc::new(Mutex::new(Some(client.new_mutated_txn()))) })
    }

    method discard(mut ctx) {
      let cb = ctx.argument::<JsFunction>(0)?;

      let this = ctx.this();
      let guard = ctx.lock();

      let txn_arc_mutex = this.borrow(&guard).txn.clone();
      Arc::downgrade(&txn_arc_mutex);

      smol::run(async {
        use dgraph_tonic::Mutate;
        use dgraph_tonic::DgraphError;

        let mut mutex_guard = txn_arc_mutex.lock().unwrap();
        let txn = mutex_guard.take();

        let result = match txn {
          Some(t) => t.discard().await,
          None => Err(DgraphError::EmptyTxn)
        };

        let task = DgraphCallbackTask {
          result: Mutex::new(Some(result)),
        };

        task.schedule(cb);
      });

      Ok(ctx.undefined().upcast())
    }

    method mutate(mut ctx) {
      let mutation = ctx.argument::<JsMutation>(0)?;
      let cb = ctx.argument::<JsFunction>(1)?;

      let this = ctx.this();
      let guard = ctx.lock();

      let mu = mutation.borrow(&guard).clone();

      let txn_arc_mutex = this.borrow(&guard).txn.clone();
      Arc::downgrade(&txn_arc_mutex);

      smol::run(async {
        use dgraph_tonic::Mutate;
        use dgraph_tonic::DgraphError;

        let mut mutex_guard = txn_arc_mutex.lock().unwrap();
        let txn = mutex_guard.as_mut();

        let response = match txn {
          Some(t) => t.mutate(mu.clone()).await,
          None => Err(DgraphError::EmptyTxn)
        };

        let task = ResponseTask {
          response: Mutex::new(Some(response)),
        };

        task.schedule(cb);
      });

      Ok(ctx.undefined().upcast())
    }

    method commit(mut ctx) {
      let cb = ctx.argument::<JsFunction>(0)?;

      let this = ctx.this();
      let guard = ctx.lock();

      let txn_arc_mutex = this.borrow(&guard).txn.clone();
      Arc::downgrade(&txn_arc_mutex);

      smol::run(async {
        use dgraph_tonic::Mutate;
        use dgraph_tonic::DgraphError;

        let mut mutex_guard = txn_arc_mutex.lock().unwrap();
        let txn = mutex_guard.take();

        let result = match txn {
          Some(t) => t.commit().await,
          None => Err(DgraphError::EmptyTxn)
        };

        let task = DgraphCallbackTask {
          result: Mutex::new(Some(result)),
        };

        task.schedule(cb);
      });

      Ok(ctx.undefined().upcast())
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
