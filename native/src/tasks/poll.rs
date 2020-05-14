use neon::prelude::*;

use std::sync::mpsc::{self, RecvTimeoutError};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use crate::classes::ResponseEventWrapper;

use crate::js::JsResponse;
use crate::utils::hashmap_to_jsobject;

pub struct PollTask {
  pub rx: Arc<Mutex<mpsc::Receiver<ResponseEventWrapper>>>,
}

impl Task for PollTask {
  type Output = ResponseEventWrapper;
  type Error = RecvTimeoutError;
  type JsEvent = JsValue;

  fn perform(&self) -> Result<Self::Output, Self::Error> {
    let duration = Duration::from_millis(50);
    let rx = self.rx.lock().unwrap();
    rx.recv_timeout(duration)
  }

  fn complete(self, mut ctx: TaskContext, result: Result<Self::Output, Self::Error>) -> JsResult<Self::JsEvent> {
    match result {
      Ok(responseEvent) => match responseEvent.result {
        Ok(x) => {
          let json_js_string = ctx.string(std::str::from_utf8(&x.json).unwrap()).upcast();
          let uids_map = hashmap_to_jsobject(&mut ctx, &x.uids)?.upcast();

          let obj = ctx.empty_object();
          let resp_id = ctx.string(responseEvent.resp_id);
          obj.set(&mut ctx, "id", resp_id);

          let response = JsResponse::new::<_, JsValue, _>(
            &mut ctx,
            vec![
              json_js_string,
              uids_map,
            ],
          )?;

          obj.set(&mut ctx, "response", response);

          Ok(obj.upcast())
        },
        Err(e) => ctx.throw_error(format!("Txn Error - {:?}", e))
      },
      Err(RecvTimeoutError::Timeout) => ctx.throw_error(format!("Poll Timeout Error - {:?}", RecvTimeoutError::Timeout)),
      Err(RecvTimeoutError::Disconnected) => ctx.throw_error(format!("Channel Disconnect Error - {:?}", RecvTimeoutError::Disconnected)),
    }
  }
}
