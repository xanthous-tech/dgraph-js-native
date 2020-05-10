extern crate neon;

use std::sync::{Arc, Mutex};

use neon::prelude::*;

use dgraph_tonic::sync::{Client};

pub mod classes;
pub mod tasks;
pub mod utils;

use classes::{DgraphClientWrapper, ReadOnlyQueryTxnWrapper, BestEffortQueryTxnWrapper};
use tasks::{QueryWithVarsTask, QueryTask};
use utils::convert_js_vars_object;

declare_types! {
  pub class JsDgraphClient for DgraphClientWrapper {
    init (mut ctx) {
      let servers_array_handle: Handle<JsArray> = ctx.argument::<JsArray>(0)?;
      let servers_jsvalue_vec: Vec<Handle<JsValue>> = servers_array_handle.to_vec(&mut ctx)?;
      let servers: Vec<String> = servers_jsvalue_vec.iter()
        .filter(|&value| value.is_a::<JsString>())
        .map(|&value| value.downcast::<JsString>())
        .map(|value| value.or_throw(&mut ctx).unwrap().value())
        .collect::<Vec<_>>();

      let client = Client::new(servers).expect("dgraph client");

      Ok(DgraphClientWrapper { client: client })
    }

    method newQueryTxn(mut ctx) {
      let this: Handle<JsDgraphClient> = ctx.this();
      let is_best_effort = ctx.argument::<JsBoolean>(0)?.value();
      if is_best_effort {
        Ok(JsBestEffortTxn::new(&mut ctx, vec![this])?.upcast())
      } else {
        Ok(JsReadOnlyTxn::new(&mut ctx, vec![this])?.upcast())
      }
    }
  }

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

      let task = QueryTask {
        txn: txn,
        query: query,
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

      let task = QueryWithVarsTask {
        txn: txn,
        query: query,
        vars: convert_js_vars_object(&mut ctx, vars_obj).unwrap(),
      };

      task.schedule(cb);

      Ok(ctx.undefined().upcast())
    }
  }

  pub class JsBestEffortTxn for BestEffortQueryTxnWrapper {
    init(mut ctx) {
      let client = ctx.argument::<JsDgraphClient>(0)?;
      let guard = ctx.lock();
      let client = client.borrow(&guard);

      Ok(BestEffortQueryTxnWrapper { txn: Arc::new(Mutex::new(client.new_best_effort_txn())) })
    }

    method query(mut ctx) {
      let query = ctx.argument::<JsString>(0)?.value();
      let cb = ctx.argument::<JsFunction>(1)?;

      let this = ctx.this();
      let guard = ctx.lock();

      let txn = this.borrow(&guard).txn.clone();

      let task = QueryTask {
        txn: txn,
        query: query,
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

      let task = QueryWithVarsTask {
        txn: txn,
        query: query,
        vars: convert_js_vars_object(&mut ctx, vars_obj).unwrap(),
      };

      task.schedule(cb);

      Ok(ctx.undefined().upcast())
    }
  }
}

register_module!(mut ctx, {
  ctx.export_class::<JsDgraphClient>("Client")?;
  ctx.export_class::<JsReadOnlyTxn>("ReadOnlyTxn")?;
  ctx.export_class::<JsBestEffortTxn>("BestEffortTxn")?;

  Ok(())
});
