use dgraph_tonic::{Operation, Payload};
use dgraph_tonic::{Client, TxnReadOnly, TxnBestEffort, TxnMutated};

pub struct DgraphClientWrapper {
  pub client: Client,
}

impl DgraphClientWrapper {
  pub fn alter(&self, op: Operation) -> Payload {
    // TODO: consider making this async using tokio runtime
    smol::run(async {
      self.client.alter(op).await.expect("client alter failed")
    })
  }

  pub fn new_read_only_txn(&self) -> TxnReadOnly {
    self.client.new_read_only_txn()
  }

  pub fn new_best_effort_txn(&self) -> TxnBestEffort {
    self.client.new_best_effort_txn()
  }

  pub fn new_mutated_txn(&self) ->  TxnMutated {
    self.client.new_mutated_txn()
  }
}
