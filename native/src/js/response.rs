use neon::prelude::*;

use crate::classes::ResponseWrapper;
use crate::utils::{jsobject_to_hashmap, convert_value, hashmap_to_jsobject};

declare_types! {
  pub class JsResponse for ResponseWrapper {
    init(mut ctx) {
      let json_string = ctx.argument::<JsString>(0)?.value();
      let json = json_string;
      let uids_obj = ctx.argument::<JsObject>(1)?;
      let uids_map = jsobject_to_hashmap(&mut ctx, uids_obj)?;

      // TODO: latency struct

      Ok(ResponseWrapper { json, uids_map })
    }

    method getJson(mut ctx) {
      let this = ctx.this();
      let guard = ctx.lock();
      let value = this.borrow(&guard).get_json_value().unwrap_or_default();

      Ok(convert_value(&mut ctx, &value))
    }

    method getUidsMap(mut ctx) {
      let this = ctx.this();
      let guard = ctx.lock();
      let uids_map = this.borrow(&guard).uids_map.clone();

      Ok(hashmap_to_jsobject(&mut ctx, &uids_map)?.upcast())
    }
  }
}
