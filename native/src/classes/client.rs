use dgraph_tonic::{Operation, Payload, LazyClient, LazyDefaultChannel};
use dgraph_tonic::{Client, ReadOnlyTxn, BestEffortTxn, MutatedTxn};

pub struct DgraphClientWrapper {
  pub client: Client,
}

impl DgraphClientWrapper {
  pub fn alter(&self, op: Operation) -> Payload {
    smol::run(async {
      self.client.alter(op).await.expect("client alter failed")
    })
  }

  pub fn new_read_only_txn(&self) -> ReadOnlyTxn<LazyClient<LazyDefaultChannel>> {
    self.client.new_read_only_txn()
  }

  pub fn new_best_effort_txn(&self) -> BestEffortTxn<LazyClient<LazyDefaultChannel>> {
    self.client.new_best_effort_txn()
  }

  pub fn new_mutated_txn(&self) ->  MutatedTxn<LazyClient<LazyDefaultChannel>> {
    self.client.new_mutated_txn()
  }
}
