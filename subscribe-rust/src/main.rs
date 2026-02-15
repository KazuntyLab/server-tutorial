use std::net::ToSocketAddrs;

use capnp::capability::Rc;
use capnp_rpc::{rpc_twoparty_capnp, twoparty, RpcSystem};
use futures::{AsyncReadExt, FutureExt};
use tokio::net::TcpStream;

pub mod swipe_capnp;

struct SwipeListenerImpl;

impl swipe_capnp::swipe_listener::Server for SwipeListenerImpl {
    // Rc<SwipeListenerImpl>で色々な人がこれを使う度に参照カウンタを増やし、最後に使い終わったらメモリから消す
    fn on_swipe(
        self: Rc<SwipeListenerImpl>,
        params: swipe_capnp::swipe_listener::OnSwipeParams,
        _results: swipe_capnp::swipe_listener::OnSwipeResults,
    ) -> impl futures::Future<Output = Result<(), capnp::Error>> {
        // 自動生成コードから読み取る
        let reader = params.get().unwrap();
        let event = reader.get_event().unwrap();
        let direction = event.get_direction().unwrap();
        let dir_str = match direction {
            swipe_capnp::Direction::Left => "左",
            swipe_capnp::Direction::Right => "右",
        };
        println!("[Rust] 通知受信！ スワイプ: {}", dir_str);
        capnp::capability::Promise::ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Box<dyn std::error::Error>でToSocketAddrs、TcpStream、capnp::Errorのエラーをまとめて扱う
    let addr = "127.0.0.1:9000".to_socket_addrs()?.next().unwrap();
    println!("[Rust] C++サーバー ({}) に接続中...", addr);

    // ?でエラーが発生したら、Boxでエラーを返してくれる
    let stream = TcpStream::connect(&addr).await?;
    let (reader, writer) = tokio_util::compat::TokioAsyncReadCompatExt::compat(stream).split();

    let network = twoparty::VatNetwork::new(
        reader,
        writer,
        rpc_twoparty_capnp::Side::Client,
        Default::default(),
    );
    let mut rpc_system = RpcSystem::new(Box::new(network), None);

    let client: swipe_capnp::swipe_service::Client =
        rpc_system.bootstrap(rpc_twoparty_capnp::Side::Server);
    let listener = capnp_rpc::new_client(SwipeListenerImpl);
    let mut request = client.subscribe_request();
    request.get().set_listener(listener);

    println!("[Rust] サーバーに登録完了。待機します...");
    let _ = request.send().promise.await?;
    tokio::task::spawn_local(rpc_system.map(|_| ()));

    futures::future::pending::<()>().await;
    Ok(())
}

// /// Copyをつけたため、Directionは渡すときに自動でコピーされる
// #[derive(Debug, Clone, Copy)]
// enum Direction {
//     Left,
//     Right,
// }

// /// traitで抽象関数を定義
// trait SwipeListener {
//     fn on_notify(&self, dir: Direction);
// }

// /// structで構造体を定義
// struct RustSubscriber;
// /// implで抽象関数の具体化
// impl SwipeListener for RustSubscriber {
//     fn on_notify(&self, _dir: Direction) {
//         let dir_str = match _dir {
//             Direction::Left => "左",
//             Direction::Right => "右",
//         };
//         println!("Rust: {}に動きました！", dir_str);
//     }
// }

// struct Rust2Subscriber;
// impl SwipeListener for Rust2Subscriber {
//     fn on_notify(&self, _dir: Direction) {
//         let dir_str = match _dir {
//             Direction::Left => "左",
//             Direction::Right => "右",
//         };
//         println!("Rust2: {}に動きました！", dir_str);
//     }
// }

// struct SwipeServer<'a> {
//     subscrivers: Vec<&'a dyn SwipeListener>,
// }

// // 'aで、SwipeListenerの生存期間にSwipeServerを合わせる
// impl<'a> SwipeServer<'a> {
//     // &mut selfは中身の書き換えが可能
//     // dynで実行時に、どのstructになるか決まりますよ
//     fn add_subscriber(&mut self, s: &'a dyn SwipeListener) {
//         self.subscrivers.push(s);
//     }

//     // &selfは中身の参照のみ
//     fn push_swipe(&self, dir: Direction) {
//         println!("サーバー: スワイプを受信。全サブスクライバへ通知します...");
//         for s in &self.subscrivers {
//             s.on_notify(dir);
//         }
//     }
// }

// fn main() {
//     let mut server = SwipeServer {
//         subscrivers: Vec::new(),
//     };
//     let sub1 = RustSubscriber;
//     let sub2 = Rust2Subscriber;

//     // mutなので、中のsubscriberrsを書き換えることが可能
//     server.add_subscriber(&sub1);
//     server.add_subscriber(&sub2);

//     server.push_swipe(Direction::Right);
//     server.push_swipe(Direction::Left);
// }

// // int b = a;でaは空っぽになる
// // int b = &a;で参照する。aは死なない
