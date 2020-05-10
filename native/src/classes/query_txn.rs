use std::sync::{Arc, Mutex};

use dgraph_tonic::{LazyClient, LazyDefaultChannel};
use dgraph_tonic::sync::{Query, ReadOnlyTxn, BestEffortTxn};

pub struct QueryTxnWrapper<Q: Query> {
  pub txn: Arc<Mutex<Q>>,
}

impl<Q> QueryTxnWrapper<Q> where Q: Query {
  pub fn new(txn: Q) -> QueryTxnWrapper<Q> {
    QueryTxnWrapper { txn: Arc::new(Mutex::new(txn)) }
  }
}

// TODO: this currently only works with un-authenticated channel
// need concrete type to bypass neon declare_types macro
pub type ReadOnlyQueryTxnWrapper = QueryTxnWrapper<ReadOnlyTxn<LazyClient<LazyDefaultChannel>>>;
pub type BestEffortQueryTxnWrapper = QueryTxnWrapper<BestEffortTxn<LazyClient<LazyDefaultChannel>>>;
