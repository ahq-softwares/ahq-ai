use std::sync::Arc;

use crate::structs::LlamaServer;

mod structs;

#[tokio::main]
async fn main() {
  let data: Arc<str> = Arc::from(String::from(""));

  let server = LlamaServer::new(data, 8080);
}
