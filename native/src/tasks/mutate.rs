use std::sync::{Arc, Mutex};

use neon::prelude::*;

use dgraph_tonic::{DgraphError, Mutation, MutationResponse};
use dgraph_tonic::sync::{Mutate};

use crate::js::JsResponse;
use crate::utils::hashmap_to_jsobject;

pub struct MutateTask<M> where M: Mutate {
  pub txn: Arc<Mutex<Option<M>>>,
  pub mu: Mutation,
}

impl<M> Task for MutateTask<M> where M: Mutate + 'static {
  type Output = MutationResponse;
  type Error = DgraphError;
  type JsEvent = JsResponse;

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
      Ok(x) => {
        let json_js_string = ctx.string(std::str::from_utf8(&x.json).unwrap()).upcast();
        let uids_map = hashmap_to_jsobject(&mut ctx, &x.uids)?.upcast();

        JsResponse::new::<_, JsValue, _>(
          &mut ctx,
          vec![
            json_js_string,
            uids_map,
          ],
        )
      },
      Err(e) => ctx.throw_error(format!("MutateTask Error - {:?}", e))
    }
  }
}
