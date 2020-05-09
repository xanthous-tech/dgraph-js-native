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
