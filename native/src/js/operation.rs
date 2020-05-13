use neon::prelude::*;

use dgraph_tonic::Operation;

declare_types! {
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
}
