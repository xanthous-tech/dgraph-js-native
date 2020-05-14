use dgraph_tonic::{Client, Query};

#[tokio::main]
async fn main() {
  let client = Client::new("http://localhost:9080").expect("dgraph client");

  let mut txn = client.new_read_only_txn();

  let q = r#"{
    total(func: eq(dgraph.type, "Node")) {
      count(uid)
    }
  }"#;

  let response = txn.query(q).await.expect("query");

  println!("response - {:?}", response);
}
