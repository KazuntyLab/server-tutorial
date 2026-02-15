use capnp_rpc::{rpc_twoparty_capnp, twoparty, RpcSystem};
use futures::AsyncReadExt;
use futures::FutureExt;
use std::net::ToSocketAddrs;
use tokio::net::TcpStream;

pub mod swipe_capnp;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:9000".to_socket_addrs()?.next().unwrap();
    println!("[Rust-Publish] C++サーバー ({}) に接続中...", addr);

    let stream = TcpStream::connect(&addr).await?;
    let (reader, writer) = tokio_util::compat::TokioAsyncReadCompatExt::compat(stream).split();

    let local = tokio::task::LocalSet::new();
    local
        .run_until(async move {
            let network = twoparty::VatNetwork::new(
                reader,
                writer,
                rpc_twoparty_capnp::Side::Client,
                Default::default(),
            );
            let mut rpc_system = RpcSystem::new(Box::new(network), None);

            let client: swipe_capnp::swipe_service::Client =
                rpc_system.bootstrap(rpc_twoparty_capnp::Side::Server);
            tokio::task::spawn_local(rpc_system.map(|_| ()));

            println!("[Rust-Publish] スワイプイベント(RIGHT)を送信中...");
            let mut request = client.push_swipe_request();
            let mut event = request.get().init_event();
            event.set_direction(swipe_capnp::Direction::Right);

            request.send().promise.await?;
            println!("[Rust-Publish] 送信完了！");

            Ok::<(), Box<dyn std::error::Error>>(())
        })
        .await?;

    Ok(())
}
