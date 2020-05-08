pub mod json;

use neon::prelude::*;

use serde_json::Value;

use std::str::from_utf8;
use std::collections::HashMap;

use dgraph_tonic::{DgraphError};
use dgraph_tonic::sync::{Client, Query, TxnVariant, TxnState};

// struct QueryWithVarsTask {
//   txn: TxnVariant<TxnState>,
//   query: String,
//   vars: HashMap<String, String>,
// }

// impl Task for QueryWithVarsTask {
//   type Output = Value;
//   type Error = DgraphError;
//   type JsEvent = JsValue;

//   fn perform(&self) -> Result<Self::Output, Self::Error> {
//     let response = self.txn.query_with_vars(self.query, self.vars)?;

//     let json_str = from_utf8(&response.json).unwrap_or_default();
//     let value: Value = serde_json::from_str(json_str).unwrap_or_default();

//     Ok(value)
//   }

//   fn complete(self, mut cx: TaskContext, result: Result<Self::Output, Self::Error>) -> JsResult<Self::JsEvent> {
//     Ok(convert_value(cx, result.unwrap()).unwrap())
//   }
// }

fn convert_value<'a>(ctx: &mut CallContext<'a, JsDgraphClient>, value: &Value) -> Result<Handle<'a, JsValue>, &'a str> {
  match value {
    Value::Null => Ok(ctx.null().upcast()),
    Value::Bool(b) => Ok(ctx.boolean(*b).upcast()),
    Value::Number(n) => Ok(ctx.number(n.as_f64().unwrap()).upcast()),
    Value::String(s) => Ok(ctx.string(s).upcast()),
    Value::Array(a) => {
      let js_array = JsArray::new(ctx, a.len() as u32);
      for (i, json_value) in a.iter().enumerate() {
        let js_value = convert_value(ctx, json_value).unwrap();
        js_array.set(ctx, i as u32, js_value).unwrap();
      }
      Ok(js_array.upcast())
    },
    Value::Object(o) => {
      let js_object = JsObject::new(ctx);
      for (_, key) in o.keys().enumerate() {
        let js_value = convert_value(ctx, o.get(key).unwrap()).unwrap();
        js_object.set(ctx, key.as_str(), js_value).unwrap();
      }
      Ok(js_object.upcast())
    },
  }
}

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

      let this = ctx.this();
      let guard = ctx.lock();

      let mut txn = this.borrow(&guard).new_read_only_txn();

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

      let response = txn.query_with_vars(query, vars).unwrap();

      let json_str = from_utf8(&response.json).unwrap_or_default();
      let value: Value = serde_json::from_str(json_str).unwrap_or_default();

      // let task = QueryWithVarsTask {
      //   txn: txn,
      //   query: query,
      //   vars: HashMap::new(),
      // };

      Ok(convert_value(&mut ctx, &value).unwrap())
    }
  }
}

register_module!(mut cx, {
  cx.export_class::<JsDgraphClient>("Client")?;

  Ok(())
});
