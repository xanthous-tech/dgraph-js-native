use std::sync::{Arc, Mutex};

use crate::utils::convert_uids_map;

use neon::prelude::*;

use dgraph_tonic::{DgraphError, Mutation, MutationResponse};
use dgraph_tonic::sync::{Mutate};

pub struct MutateTask<M> where M: Mutate {
  pub txn: Arc<Mutex<Option<M>>>,
  pub mu: Mutation,
}

impl<M> Task for MutateTask<M> where M: Mutate + 'static {
  type Output = MutationResponse;
  type Error = DgraphError;
  type JsEvent = JsValue;

  fn perform(&self) -> Result<Self::Output, Self::Error> {
    let mut mutex_guard = self.txn.lock().unwrap();
    let txn = mutex_guard.as_mut();
    match txn {
      Some(t) => t.mutate(self.mu.clone()),
      None => Err(DgraphError::EmptyTxn)
    }
  }

  fn complete(self, mut ctx: TaskContext, result: Result<Self::Output, Self::Error>) -> JsResult<Self::JsEvent> {
    match result {
      Ok(x) => Ok(convert_uids_map(&mut ctx, &x.uids).unwrap().upcast()),
      Err(e) => ctx.throw_error(format!("MutateTask Error - {:?}", e))
    }
  }
}
