extern crate neon;

use neon::prelude::*;

pub mod classes;
pub mod tasks;
pub mod utils;
pub mod js;

register_module!(mut ctx, {
  ctx.export_class::<js::JsResponse>("Response")?;
  ctx.export_class::<js::JsOperation>("Operation")?;
  ctx.export_class::<js::JsMutation>("Mutation")?;
  ctx.export_class::<js::JsDgraphClient>("Client")?;
  ctx.export_class::<js::JsReadOnlyTxn>("ReadOnlyTxn")?;
  ctx.export_class::<js::JsBestEffortTxn>("BestEffortTxn")?;
  ctx.export_class::<js::JsMutatedTxn>("MutatedTxn")?;

  Ok(())
});
