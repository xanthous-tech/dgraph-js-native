use std::str::from_utf8;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use neon::prelude::*;
use serde_json::Value;

use crate::utils::convert_value;

use dgraph_tonic::{DgraphError, LazyClient, LazyDefaultChannel};
use dgraph_tonic::sync::{Query, ReadOnlyTxn};

pub struct QueryWithVarsTask {
  pub txn: Arc<Mutex<ReadOnlyTxn<LazyClient<LazyDefaultChannel>>>>,
  pub query: String,
  pub vars: HashMap<String, String>,
}

impl Task for QueryWithVarsTask {
  type Output = Value;
  type Error = DgraphError;
  type JsEvent = JsValue;

  fn perform(&self) -> Result<Self::Output, Self::Error> {
    let response = self.txn.lock().unwrap().query_with_vars(self.query.clone(), self.vars.clone())?;

    let json_str = from_utf8(&response.json).unwrap_or_default();
    let value: Value = serde_json::from_str(json_str).unwrap_or_default();

    Ok(value)
  }

  fn complete(self, mut cx: TaskContext, result: Result<Self::Output, Self::Error>) -> JsResult<Self::JsEvent> {
    Ok(convert_value(&mut cx, &result.unwrap()).unwrap())
  }
}
