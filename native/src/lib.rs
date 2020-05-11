extern crate neon;

use std::string::String;
use std::sync::{Arc, Mutex};

use neon::prelude::*;

use dgraph_tonic::{Mutation, Operation};
use dgraph_tonic::sync::{Client};

pub mod classes;
pub mod tasks;
pub mod utils;

use classes::{DgraphClientWrapper, ReadOnlyQueryTxnWrapper, BestEffortQueryTxnWrapper, MutatedTxnWrapper};
use tasks::{QueryWithVarsTask, QueryTask, MutateTask, CommitTask};
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

    method newMutateTxn(mut ctx) {
      let this: Handle<JsDgraphClient> = ctx.this();
      Ok(JsMutatedTxn::new(&mut ctx, vec![this])?.upcast())
    }

    method alter(mut ctx) {
      let this: Handle<JsDgraphClient> = ctx.this();
      let operation = ctx.argument::<JsOperation>(0)?;
      let guard = ctx.lock();

      let payload = this.borrow(&guard).alter(operation.borrow(&guard).clone());
      let data = payload.data;

      Ok(ctx.string(String::from_utf8(data).unwrap()).upcast())
    }
  }

  pub class JsOperation for Operation {
    init(_) {
      Ok(Operation { ..Default::default() })
    }

    method setSchema(mut ctx) {
      let schema_string = ctx.argument::<JsString>(0)?.value();

      let mut this = ctx.this();
      let guard = ctx.lock();
      this.borrow_mut(&guard).schema = schema_string;

      Ok(ctx.undefined().upcast())
    }
  }

  pub class JsMutatedTxn for MutatedTxnWrapper {
    init(mut ctx) {
      let client = ctx.argument::<JsDgraphClient>(0)?;
      let guard = ctx.lock();
      let client = client.borrow(&guard);

      Ok(MutatedTxnWrapper { txn: Arc::new(Mutex::new(Some(client.new_mutated_txn()))) })
    }

    method mutate(mut ctx) {
      let mutation = ctx.argument::<JsMutation>(0)?;
      let cb = ctx.argument::<JsFunction>(1)?;

      let this = ctx.this();
      let guard = ctx.lock();

      let txn = this.borrow(&guard).txn.clone();
      Arc::downgrade(&txn);
      let mu = mutation.borrow(&guard).clone();

      let task = MutateTask {
        txn,
        mu,
      };

      task.schedule(cb);

      Ok(ctx.undefined().upcast())
    }

    method commit(mut ctx) {
      let cb = ctx.argument::<JsFunction>(0)?;

      let this = ctx.this();
      let guard = ctx.lock();

      let txn = this.borrow(&guard).txn.clone();
      Arc::downgrade(&txn);

      let task = CommitTask {
        txn,
      };

      task.schedule(cb);

      Ok(ctx.undefined().upcast())
    }
  }

  pub class JsMutation for Mutation {
    init(_) {
      Ok(Mutation::new())
    }

    method setSetJson(mut ctx) {
      let set_json_string = ctx.argument::<JsString>(0)?.value();

      let mut this = ctx.this();
      let guard = ctx.lock();
      this.borrow_mut(&guard).set_json = set_json_string.into_bytes();

      Ok(ctx.undefined().upcast())
    }

    method setSetNquads(mut ctx) {
      let set_nquads_string = ctx.argument::<JsString>(0)?.value();

      let mut this = ctx.this();
      let guard = ctx.lock();
      this.borrow_mut(&guard).set_nquads = set_nquads_string.into_bytes();

      Ok(ctx.undefined().upcast())
    }

    method setDeleteJson(mut ctx) {
      let delete_json_string = ctx.argument::<JsString>(0)?.value();

      let mut this = ctx.this();
      let guard = ctx.lock();
      this.borrow_mut(&guard).delete_json = delete_json_string.into_bytes();

      Ok(ctx.undefined().upcast())
    }

    method setDelNquads(mut ctx) {
      let del_nquads_string = ctx.argument::<JsString>(0)?.value();

      let mut this = ctx.this();
      let guard = ctx.lock();
      this.borrow_mut(&guard).del_nquads = del_nquads_string.into_bytes();

      Ok(ctx.undefined().upcast())
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

register_module!(mut ctx, {
  ctx.export_class::<JsDgraphClient>("Client")?;
  ctx.export_class::<JsReadOnlyTxn>("ReadOnlyTxn")?;
  ctx.export_class::<JsBestEffortTxn>("BestEffortTxn")?;
  ctx.export_class::<JsMutatedTxn>("MutatedTxn")?;
  ctx.export_class::<JsMutation>("Mutation")?;

  Ok(())
});
