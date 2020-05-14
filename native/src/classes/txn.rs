use std::collections::HashMap;
use std::string::String;
use std::sync::{Arc, Mutex as StdMutex};
use std::thread;

use tokio_global::AutoRuntime;
use tokio::sync::Mutex;
use tokio::sync::mpsc;
use nanoid::nanoid;

use dgraph_tonic::{LazyClient, LazyDefaultChannel, DgraphError, Mutation, Response};
use dgraph_tonic::{Query, Mutate, ReadOnlyTxn, BestEffortTxn, MutatedTxn};

use super::event::ResponseEventWrapper;

pub struct QueryTxnWrapper<Q: Query> {
  pub txn: Arc<Mutex<Option<Q>>>,
  pub response_tx: mpsc::UnboundedSender<ResponseEventWrapper>,
  pub response_rx: Arc<StdMutex<mpsc::UnboundedReceiver<ResponseEventWrapper>>>,
}

impl<Q> QueryTxnWrapper<Q> where Q: Query + 'static {
  pub fn query(&self, query: String) -> String {
    let txn_arc_mutex = self.txn.clone();
    Arc::downgrade(&txn_arc_mutex);

    let txn_id = nanoid!();
    let resp_id = txn_id.clone();
    let tx = self.response_tx.clone();

    async move {
      let mut txn_guard = txn_arc_mutex.lock().await;
      let txn = txn_guard.as_mut();

      let response = match txn {
        Some(t) => t.query(query.clone()).await,
        None => Err(DgraphError::EmptyTxn)
      };

      match tx.send(ResponseEventWrapper {
        resp_id: txn_id.clone(),
        result: response,
      }) {
        Ok(()) => (),
        Err(_) => ()
      }
    }.spawn();

    resp_id
  }

  pub fn query_with_vars(&self, query: String, vars: HashMap<String, String>) -> String {
    let txn_arc_mutex = self.txn.clone();
    Arc::downgrade(&txn_arc_mutex);

    let txn_id = nanoid!();
    let resp_id = txn_id.clone();
    let tx = self.response_tx.clone();

    async move {
      let mut txn_guard = txn_arc_mutex.lock().await;
      let txn = txn_guard.as_mut();

      let response = match txn {
        Some(t) => t.query_with_vars(query.clone(), vars.clone()).await,
        None => Err(DgraphError::EmptyTxn)
      };

      match tx.send(ResponseEventWrapper {
        resp_id: txn_id.clone(),
        result: response,
      }) {
        Ok(()) => (),
        Err(_) => ()
      }
    }.spawn();

    resp_id
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
  pub response_tx: mpsc::UnboundedSender<ResponseEventWrapper>,
  pub response_rx: Arc<StdMutex<mpsc::UnboundedReceiver<ResponseEventWrapper>>>,
}

impl<M> MutateTxnWrapper<M> where M: Mutate + 'static {
  pub fn discard(&self) -> String {
    let txn_arc_mutex = self.txn.clone();
    Arc::downgrade(&txn_arc_mutex);

    let txn_id = nanoid!();
    let resp_id = txn_id.clone();
    let tx = self.response_tx.clone();

    async move {
      let mut txn_guard = txn_arc_mutex.lock().await;
      let txn = txn_guard.take();

      let response = match txn {
        Some(t) => t.discard().await,
        None => Err(DgraphError::EmptyTxn)
      };

      match tx.send(ResponseEventWrapper {
        resp_id: txn_id.clone(),
        result: match response {
          Ok(()) => Ok(Response { json: String::from("{\"type\": \"discard\"}").into_bytes(), ..Default::default() }),
          Err(e) => Err(e),
        },
      }) {
        Ok(()) => (),
        Err(_) => ()
      }
    }.spawn();

    resp_id
  }

  pub fn commit(&self) -> String {
    let txn_arc_mutex = self.txn.clone();
    Arc::downgrade(&txn_arc_mutex);

    let txn_id = nanoid!();
    let resp_id = txn_id.clone();
    let tx = self.response_tx.clone();

    async move {
      let mut txn_guard = txn_arc_mutex.lock().await;
      let txn = txn_guard.take();

      let response = match txn {
        Some(t) => t.commit().await,
        None => Err(DgraphError::EmptyTxn)
      };

      match tx.send(ResponseEventWrapper {
        resp_id: txn_id.clone(),
        result: match response {
          Ok(()) => Ok(Response { json: String::from("{\"type\": \"commit\"}").into_bytes(), ..Default::default() }),
          Err(e) => Err(e),
        },
      }) {
        Ok(()) => (),
        Err(_) => ()
      }
    }.spawn();

    resp_id
  }

  pub fn mutate(&self, mu: Mutation) -> String {
    let txn_arc_mutex = self.txn.clone();
    Arc::downgrade(&txn_arc_mutex);

    let txn_id = nanoid!();
    let resp_id = txn_id.clone();
    let tx = self.response_tx.clone();

    thread::spawn(move || smol::run(async {
      let mut txn_guard = txn_arc_mutex.lock().await;
      let txn = txn_guard.as_mut();

      let response = match txn {
        Some(t) => t.mutate(mu.clone()).await,
        None => Err(DgraphError::EmptyTxn)
      };

      match tx.send(ResponseEventWrapper {
        resp_id: txn_id.clone(),
        result: response,
      }) {
        Ok(()) => (),
        Err(_) => ()
      }
    }));

    resp_id
  }

  pub fn upsert(&self, query: String, mu: Mutation) -> String {
    let txn_arc_mutex = self.txn.clone();
    Arc::downgrade(&txn_arc_mutex);

    let txn_id = nanoid!();
    let resp_id = txn_id.clone();
    let tx = self.response_tx.clone();

    async move {
      let mut txn_guard = txn_arc_mutex.lock().await;
      let txn = txn_guard.take();

      let response = match txn {
        Some(t) => t.upsert(query.clone(), mu.clone()).await,
        None => Err(DgraphError::EmptyTxn)
      };

      match tx.send(ResponseEventWrapper {
        resp_id: txn_id.clone(),
        result: response,
      }) {
        Ok(()) => (),
        Err(_) => ()
      }
    }.spawn();

    resp_id
  }

  pub fn upsert_with_vars(&self, query: String, vars: HashMap<String, String>, mu: Mutation) -> String {
    let txn_arc_mutex = self.txn.clone();
    Arc::downgrade(&txn_arc_mutex);

    let txn_id = nanoid!();
    let resp_id = txn_id.clone();
    let tx = self.response_tx.clone();

    async move {
      let mut txn_guard = txn_arc_mutex.lock().await;
      let txn = txn_guard.take();

      let response = match txn {
        Some(t) => t.upsert_with_vars(query.clone(), vars.clone(), mu.clone()).await,
        None => Err(DgraphError::EmptyTxn)
      };

      match tx.send(ResponseEventWrapper {
        resp_id: txn_id.clone(),
        result: response,
      }) {
        Ok(()) => (),
        Err(_) => ()
      }
    }.spawn();

    resp_id
  }

  pub fn query(&self, query: String) -> String {
    let txn_arc_mutex = self.txn.clone();
    Arc::downgrade(&txn_arc_mutex);

    let txn_id = nanoid!();
    let resp_id = txn_id.clone();
    let tx = self.response_tx.clone();

    async move {
      let mut txn_guard = txn_arc_mutex.lock().await;
      let txn = txn_guard.as_mut();

      let response = match txn {
        Some(t) => t.query(query.clone()).await,
        None => Err(DgraphError::EmptyTxn)
      };

      match tx.send(ResponseEventWrapper {
        resp_id: txn_id.clone(),
        result: response,
      }) {
        Ok(()) => (),
        Err(_) => ()
      }
    }.spawn();

    resp_id
  }

  pub fn query_with_vars(&self, query: String, vars: HashMap<String, String>) -> String {
    let txn_arc_mutex = self.txn.clone();
    Arc::downgrade(&txn_arc_mutex);

    let txn_id = nanoid!();
    let resp_id = txn_id.clone();
    let tx = self.response_tx.clone();

    async move {
      let mut txn_guard = txn_arc_mutex.lock().await;
      let txn = txn_guard.as_mut();

      let response = match txn {
        Some(t) => {
          t.query_with_vars(query.clone(), vars.clone()).await
        },
        None => Err(DgraphError::EmptyTxn)
      };

      match tx.send(ResponseEventWrapper {
        resp_id: txn_id.clone(),
        result: response,
      }) {
        Ok(()) => (),
        Err(_) => ()
      }
    }.spawn();

    resp_id
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
