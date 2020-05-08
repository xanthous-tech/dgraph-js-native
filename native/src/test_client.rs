use std::collections::HashMap;
use dgraph_tonic::{Client, Query};

#[tokio::main]
async fn main() {
  let client = Client::new("http://localhost:9080").expect("dgraph client");

  let mut txn = client.new_read_only_txn();

  let q = r#"query user($userId: string) {
    var(func: uid($userId)) {
      has_user_group @filter(NOT eq(removed, true)) {
        userGroupUid as uid
        has_workspace @facets(hasPermission: CAN_WORKSPACE_READ) @facets(eq(CAN_WORKSPACE_READ, true)) {
          permitted_has_workspace as uid
        }
        has_core @facets(hasPermission: CAN_CORE_READ) @facets(eq(CAN_CORE_READ, true)) {
          permitted_has_core as uid
        }
        has_table @facets(hasPermission: CAN_TABLE_READ) @facets(eq(CAN_TABLE_READ, true)) {
          permitted_has_table as uid
        }
        has_view @facets(hasPermission: CAN_VIEW_READ) @facets(eq(CAN_VIEW_READ, true)) {
          permitted_has_view as uid
        }
      }
    }
    userGroups(func: uid(userGroupUid)) {
      uid
      removed
    }
    workspaces(func: uid(permitted_has_workspace)) {
      uid
    }
    cores(func: uid(permitted_has_core)) {
      uid
    }
    tables(func: uid(permitted_has_table)) {
      uid
    }
    views(func: uid(permitted_has_view)) {
      uid
    }
  }"#;

  let mut vars: HashMap<String, String> = HashMap::new();
  vars.insert(String::from("$userId"), String::from("0x1"));

  let response = txn.query_with_vars(q, vars).await.expect("query");

  println!("response = {:?}", String::from_utf8(response.json).unwrap());
}