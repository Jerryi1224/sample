use std::sync::Arc;
use std::sync::Mutex;

use salvo::basic_auth::{BasicAuth, BasicAuthValidator};
use salvo::prelude::*;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let acceptor = TcpListener::new("127.0.0.1:5800").bind().await;
    Server::new(acceptor).serve(route()).await;
}

#[allow(dead_code)]
#[derive(Default, Clone, Debug)]
struct Config {
    username: String,
    password: String,
}

#[handler]
async fn hello(depot: &mut Depot) -> String {
    let current_user = depot.get::<&str>("current_user").unwrap();
    format!("Hello {current_user}")
}

struct Validator;
#[async_trait]
impl BasicAuthValidator for Validator {
    async fn validate(&self, username: &str, password: &str, depot: &mut Depot) -> bool {
        depot.insert("current_user", "root");
        username == "root" && password == "pwd"
    }
}

fn route() -> Router {
    let auth_handler = BasicAuth::new(Validator);
    Router::new()
        .hoop(auth_handler)
        .handle(hello)
}
