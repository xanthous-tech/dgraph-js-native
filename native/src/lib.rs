extern crate neon;

use neon::prelude::*;
use tokio_global::Runtime;

pub mod classes;
pub mod tasks;
pub mod utils;
pub mod js;

static mut RT: Option<Runtime> = None;

register_module!(mut ctx, {
  unsafe {
    RT = Some(Runtime::default());
    let _runner = std::thread::spawn(|| {
      Runtime::run();
    });
  }

  ctx.export_class::<js::JsResponse>("Response")?;
  ctx.export_class::<js::JsOperation>("Operation")?;
  ctx.export_class::<js::JsMutation>("Mutation")?;
  ctx.export_class::<js::JsDgraphClient>("Client")?;
  ctx.export_class::<js::JsReadOnlyTxn>("ReadOnlyTxn")?;
  ctx.export_class::<js::JsBestEffortTxn>("BestEffortTxn")?;
  ctx.export_class::<js::JsMutatedTxn>("MutatedTxn")?;

  Ok(())
});
