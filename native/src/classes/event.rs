use dgraph_tonic::{Response, DgraphError};

pub struct ResponseEventWrapper {
  pub resp_id: String,
  pub result: Result<Response, DgraphError>,
}
