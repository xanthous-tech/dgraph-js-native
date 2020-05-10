use std::sync::{Arc, Mutex};

use neon::prelude::*;
use serde_json::Value;

use crate::utils::convert_value;

use dgraph_tonic::{DgraphError};
use dgraph_tonic::sync::{Query};

pub struct QueryTask<Q> where Q: Query {
  pub txn: Arc<Mutex<Q>>,
  pub query: String,
}

impl<Q> Task for QueryTask<Q> where Q: Query + 'static {
  type Output = Value;
  type Error = DgraphError;
  type JsEvent = JsValue;

  fn perform(&self) -> Result<Self::Output, Self::Error> {
    let response = self.txn.lock().unwrap().query(self.query.clone())?;
    let value: Value = serde_json::from_slice(&response.json).unwrap_or_default();
    Ok(value)
  }

  fn complete(self, mut cx: TaskContext, result: Result<Self::Output, Self::Error>) -> JsResult<Self::JsEvent> {
    Ok(convert_value(&mut cx, &result.unwrap()).unwrap())
  }
}
