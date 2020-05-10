use std::str::from_utf8;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use neon::prelude::*;
use serde_json::Value;

use crate::utils::convert_value;

use dgraph_tonic::{DgraphError};
use dgraph_tonic::sync::{Query};

pub struct QueryWithVarsTask<Q: Query> {
  pub txn: Arc<Mutex<Q>>,
  pub query: String,
  pub vars: HashMap<String, String>,
}

impl<Q: 'static + Query> Task for QueryWithVarsTask<Q> {
  type Output = Value;
  type Error = DgraphError;
  type JsEvent = JsValue;

  fn perform(&self) -> Result<Self::Output, Self::Error> {
    let response = self.txn.lock().unwrap().query_with_vars(self.query.clone(), self.vars.clone())?;

    let value: Value = response.try_into_owned().unwrap_or_default();

    Ok(value)
  }

  fn complete(self, mut cx: TaskContext, result: Result<Self::Output, Self::Error>) -> JsResult<Self::JsEvent> {
    Ok(convert_value(&mut cx, &result.unwrap()).unwrap())
  }
}
