use std::sync::{Arc, Mutex};

use dgraph_tonic::{LazyClient, LazyDefaultChannel};
use dgraph_tonic::sync::{Query, Mutate, ReadOnlyTxn, BestEffortTxn, MutatedTxn};

pub struct QueryTxnWrapper<Q: Query> {
  pub txn: Arc<Mutex<Option<Q>>>,
}

impl<Q> QueryTxnWrapper<Q> where Q: Query {
  pub fn new(txn: Q) -> QueryTxnWrapper<Q> {
    QueryTxnWrapper { txn: Arc::new(Mutex::new(Some(txn))) }
  }
}

impl<Q> Drop for QueryTxnWrapper<Q> where Q: Query {
  fn drop(&mut self) {
    // not sure if this is enough
    Arc::downgrade(&self.txn);
  }
}

pub struct MutateTxnWrapper<M: Mutate> {
  pub txn: Arc<Mutex<Option<M>>>,
}

impl<M> MutateTxnWrapper<M> where M: Mutate {
  pub fn new(txn: M) -> MutateTxnWrapper<M> {
    MutateTxnWrapper { txn: Arc::new(Mutex::new(Some(txn))) }
  }
}

impl<M> Drop for MutateTxnWrapper<M> where M: Mutate {
  fn drop(&mut self) {
    // not sure if this is enough
    Arc::downgrade(&self.txn);
  }
}

// TODO: this currently only works with un-authenticated channel
// need concrete type to bypass neon declare_types macro
pub type ReadOnlyQueryTxnWrapper = QueryTxnWrapper<ReadOnlyTxn<LazyClient<LazyDefaultChannel>>>;
pub type BestEffortQueryTxnWrapper = QueryTxnWrapper<BestEffortTxn<LazyClient<LazyDefaultChannel>>>;
pub type MutatedTxnWrapper = MutateTxnWrapper<MutatedTxn<LazyClient<LazyDefaultChannel>>>;
