use std::sync::{Arc, Mutex};

use neon::prelude::*;

use dgraph_tonic::{Response, DgraphError};
use dgraph_tonic::sync::{Query};

use crate::js::JsResponse;
use crate::utils::hashmap_to_jsobject;

pub struct QueryTask<Q> where Q: Query {
  pub txn: Arc<Mutex<Option<Q>>>,
  pub query: String,
}

impl<Q> Task for QueryTask<Q> where Q: Query + 'static {
  type Output = Response;
  type Error = DgraphError;
  type JsEvent = JsResponse;

  fn perform(&self) -> Result<Self::Output, Self::Error> {
    let mut mutex_guard = self.txn.lock().unwrap();
    let txn = mutex_guard.as_mut();

    match txn {
      Some(t) => t.query(self.query.clone()),
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
      Err(e) => ctx.throw_error(format!("QueryTask Error - {:?}", e))
    }
  }
}
