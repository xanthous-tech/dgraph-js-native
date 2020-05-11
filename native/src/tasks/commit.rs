use std::sync::{Arc, Mutex};

use neon::prelude::*;

use dgraph_tonic::{DgraphError};
use dgraph_tonic::sync::{Mutate};

pub struct CommitTask<M> where M: Mutate {
  pub txn: Arc<Mutex<Option<M>>>,
}

impl<M> Task for CommitTask<M> where M: Mutate + 'static {
  type Output = ();
  type Error = DgraphError;
  type JsEvent = JsUndefined;

  fn perform(&self) -> Result<Self::Output, Self::Error> {
    self.txn.lock().unwrap().take().unwrap().commit()
  }

  fn complete(self, mut ctx: TaskContext, _result: Result<Self::Output, Self::Error>) -> JsResult<Self::JsEvent> {
    Ok(ctx.undefined())
  }
}
