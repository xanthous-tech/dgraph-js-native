use std::collections::HashMap;

use neon::prelude::*;

use serde_json::Value;

pub fn convert_value<'a>(ctx: &mut impl Context<'a>, value: &Value) -> Result<Handle<'a, JsValue>, &'a str> {
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

pub fn convert_js_vars_object<'a>(ctx: &mut impl Context<'a>, vars_obj: Handle<'a, JsObject>) -> Result<HashMap<String, String>, neon::result::Throw> {
  // NOTE: this operation filters out everything that is not Map<string, string> in JS.

  let mut vars: HashMap<String, String> = HashMap::new();
  let keys_vec = vars_obj.get_own_property_names(ctx)?.to_vec(ctx)?;

  for key in &keys_vec {
    if !key.is_a::<JsString>() {
      continue;
    }

    let key_string = key.downcast::<JsString>().unwrap().value();
    let value = vars_obj.get(ctx, key_string.as_str())?;

    if !value.is_a::<JsString>() {
      continue;
    }

    let value_string = value.downcast::<JsString>().unwrap().value();

    vars.insert(key_string, value_string);
  }

  Ok(vars)
}
