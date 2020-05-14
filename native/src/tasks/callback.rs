use neon::prelude::*;

use std::sync::Mutex;

use dgraph_tonic::{DgraphError};

pub struct DgraphCallbackTask {
  pub result: Mutex<Option<Result<(), DgraphError>>>,
}

impl Task for DgraphCallbackTask {
  type Output = ();
  type Error = DgraphError;
  type JsEvent = JsUndefined;

  fn perform(&self) -> Result<Self::Output, Self::Error> {
    self.result.lock().unwrap().take().unwrap()
  }

  fn complete(self, mut ctx: TaskContext, result: Result<Self::Output, Self::Error>) -> JsResult<Self::JsEvent> {
    match result {
      Ok(_) => Ok(ctx.undefined()),
      // swallow empty txn error
      Err(DgraphError::EmptyTxn) => Ok(ctx.undefined()),
      Err(e) => ctx.throw_error(format!("DgraphCallbackTask Error - {:?}", e))
    }
  }
}
