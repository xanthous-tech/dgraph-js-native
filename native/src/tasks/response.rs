use neon::prelude::*;

use std::sync::Mutex;

use dgraph_tonic::{DgraphError};
use dgraph_tonic::{Response};

use crate::js::JsResponse;
use crate::utils::hashmap_to_jsobject;

pub struct ResponseTask {
  pub response: Mutex<Option<Result<Response, DgraphError>>>,
}

impl Task for ResponseTask {
  type Output = Response;
  type Error = DgraphError;
  type JsEvent = JsResponse;

  fn perform(&self) -> Result<Self::Output, Self::Error> {
    self.response.lock().unwrap().take().unwrap()
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
      Err(e) => ctx.throw_error(format!("ResponseTask Error - {:?}", e))
    }
  }
}
