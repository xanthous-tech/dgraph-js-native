use neon::prelude::*;

use dgraph_tonic::sync::{Client};

use crate::classes::DgraphClientWrapper;

use super::operation::JsOperation;
use super::txn::{JsMutatedTxn, JsReadOnlyTxn, JsBestEffortTxn};

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
}
