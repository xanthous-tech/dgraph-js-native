pub mod tasks;
pub mod utils;

use tasks::QueryWithVarsTask;

use neon::prelude::*;

use std::collections::HashMap;
use std::sync::{Mutex};

use dgraph_tonic::sync::Client;

declare_types! {
  pub class JsDgraphClient for Client {
    init (mut ctx) {
      let servers_array_handle: Handle<JsArray> = ctx.argument::<JsArray>(0)?;
      let servers_jsvalue_vec: Vec<Handle<JsValue>> = servers_array_handle.to_vec(&mut ctx)?;
      let servers: Vec<String> = servers_jsvalue_vec.iter()
        .filter(|&value| value.is_a::<JsString>())
        .map(|&value| value.downcast::<JsString>())
        .map(|value| value.or_throw(&mut ctx).unwrap().value())
        .collect::<Vec<_>>();

      Ok(Client::new(servers).expect("dgraph client"))
    }

    method queryWithVars(mut ctx) {
      let query = ctx.argument::<JsString>(0)?.value();
      let vars_obj = ctx.argument::<JsObject>(1)?;
      let cb = ctx.argument::<JsFunction>(2)?;

      let this = ctx.this();
      let guard = ctx.lock();

      let txn = this.borrow(&guard).new_read_only_txn();

      let mut vars: HashMap<String, String> = HashMap::new();
      let keys_vec = vars_obj.get_own_property_names(&mut ctx)?.to_vec(&mut ctx)?;

      for key in &keys_vec {
        if !key.is_a::<JsString>() {
          continue;
        }

        let key_string = key.downcast::<JsString>().unwrap().value();
        let value = vars_obj.get(&mut ctx, key_string.as_str()).unwrap();

        if !value.is_a::<JsString>() {
          continue;
        }

        let value_string = value.downcast::<JsString>().unwrap().value();

        vars.insert(key_string, value_string);
      }

      // let response = txn.query_with_vars(query, vars).unwrap();
      // let json_str = from_utf8(&response.json).unwrap_or_default();
      // let value: Value = serde_json::from_str(json_str).unwrap_or_default();

      let task = QueryWithVarsTask {
        txn: Mutex::new(txn),
        query: query,
        vars: vars,
      };

      task.schedule(cb);

      Ok(ctx.undefined().upcast())
    }
  }
}

register_module!(mut cx, {
  cx.export_class::<JsDgraphClient>("Client")?;

  Ok(())
});
