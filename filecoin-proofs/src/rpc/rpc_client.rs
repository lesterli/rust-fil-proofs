use hyper::rt;
use std::time::Duration;

use jsonrpc_core_client::{RpcChannel, TypedClient, RpcError};
use jsonrpc_core::futures::Future;
use jsonrpc_http_server::*;
use jsonrpc_client_transports::transports::http;


#[derive(Clone)]
pub struct RpcClient(TypedClient);

impl From<RpcChannel> for RpcClient {
    fn from(channel: RpcChannel) -> Self {
        RpcClient(channel.into())
    }
}

impl RpcClient {
    pub fn hello(&self, msg: &'static str) -> impl Future<Item = String, Error = RpcError> {
        self.0.call_method("hello", "String", (msg,))
    }

    pub fn fail(&self) -> impl Future<Item = (), Error = RpcError> {
        self.0.call_method("fail", "()", ())
    }
    
    pub fn notify(&self, value: u64) -> impl Future<Item = (), Error = RpcError> {
        self.0.notify("notify", (value,))
    }
}

fn id<T>(t: T) -> T {
    t
}

fn example() {
    // init RPC server
    let server = rpc_server::RpcServer::serve(id);
    let (tx, rx) = std::sync::mpsc::channel();

    // create connect
    let run = http::connect(&server.uri)
        .and_then(|client: rpc_client::RpcClient| {
            client.hello("http").and_then(move |result| {
                drop(client);
                let _ = tx.send(result);
                Ok(())
            })
        })
        .map_err(|e| println!("RPC Client error: {:?}", e));

    rt::run(run);

    // get response
    let result = rx.recv_timeout(Duration::from_secs(3)).unwrap();
    assert_eq!("hello http", result);
}