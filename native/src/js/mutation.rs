use neon::prelude::*;

use dgraph_tonic::Mutation;

declare_types! {
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
}
