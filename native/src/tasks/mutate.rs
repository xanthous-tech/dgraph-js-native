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
    self.txn.lock().unwrap().as_mut().unwrap().mutate(self.mu.clone())
  }

  fn complete(self, mut ctx: TaskContext, result: Result<Self::Output, Self::Error>) -> JsResult<Self::JsEvent> {
    let response = result.unwrap();
    Ok(convert_uids_map(&mut ctx, &response.uids).unwrap().upcast())
  }
}
