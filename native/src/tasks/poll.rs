use neon::prelude::*;

use std::sync::{Arc, Mutex};

use tokio::sync::mpsc;
use tokio::sync::mpsc::error::TryRecvError;

use crate::classes::ResponseEventWrapper;

use crate::js::JsResponse;
use crate::utils::hashmap_to_jsobject;

pub struct PollTask {
  pub rx: Arc<Mutex<mpsc::UnboundedReceiver<ResponseEventWrapper>>>,
}

impl Task for PollTask {
  type Output = ResponseEventWrapper;
  type Error = TryRecvError;
  type JsEvent = JsValue;

  fn perform(&self) -> Result<Self::Output, Self::Error> {
    let mut rx = self.rx.lock().unwrap();
    rx.try_recv()
  }

  fn complete(self, mut ctx: TaskContext, result: Result<Self::Output, Self::Error>) -> JsResult<Self::JsEvent> {
    match result {
      Ok(resp_event) => match resp_event.result {
        Ok(x) => {
          let json_js_string = ctx.string(std::str::from_utf8(&x.json).unwrap()).upcast();
          let uids_map = hashmap_to_jsobject(&mut ctx, &x.uids)?.upcast();

          let obj = ctx.empty_object();
          let resp_id = ctx.string(resp_event.resp_id);
          obj.set(&mut ctx, "id", resp_id)?;

          let response = JsResponse::new::<_, JsValue, _>(
            &mut ctx,
            vec![
              json_js_string,
              uids_map,
            ],
          )?;

          obj.set(&mut ctx, "response", response)?;

          Ok(obj.upcast())
        },
        Err(e) => {
          let obj = ctx.empty_object();
          let resp_id = ctx.string(resp_event.resp_id);
          let error = ctx.string(format!("Txn Error - {:?}", e));
          obj.set(&mut ctx, "id", resp_id)?;
          obj.set(&mut ctx, "error", error)?;
          Ok(obj.upcast())
        },
      },
      Err(e) => ctx.throw_error(format!("PollTask Error - {:?}", e)),
    }
  }
}
