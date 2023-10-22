pub mod config;
pub mod net;
pub mod nlp;
pub mod rpc;

#[tokio::main]
async fn main() {
    rpc::start_request_handle_loop(nlp::translate).await;
}
