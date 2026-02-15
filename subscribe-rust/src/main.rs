/// Copyをつけたため、Directionは渡すときに自動でコピーされる
#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
}

/// traitで抽象関数を定義
trait SwipeListener {
    fn on_notify(&self, dir: Direction);
}

/// structで構造体を定義
struct RustSubscriber;
/// implで抽象関数の具体化
impl SwipeListener for RustSubscriber {
    fn on_notify(&self, _dir: Direction) {
        let dir_str = match _dir {
            Direction::Left => "左",
            Direction::Right => "右",
        };
        println!("Rust: {}に動きました！", dir_str);
    }
}

struct Rust2Subscriber;
impl SwipeListener for Rust2Subscriber {
    fn on_notify(&self, _dir: Direction) {
        let dir_str = match _dir {
            Direction::Left => "左",
            Direction::Right => "右",
        };
        println!("Rust2: {}に動きました！", dir_str);
    }
}

struct SwipeServer<'a> {
    subscrivers: Vec<&'a dyn SwipeListener>,
}

// 'aで、SwipeListenerの生存期間にSwipeServerを合わせる
impl<'a> SwipeServer<'a> {
    // &mut selfは中身の書き換えが可能
    // dynで実行時に、どのstructになるか決まりますよ
    fn add_subscriber(&mut self, s: &'a dyn SwipeListener) {
        self.subscrivers.push(s);
    }

    // &selfは中身の参照のみ
    fn push_swipe(&self, dir: Direction) {
        println!("サーバー: スワイプを受信。全サブスクライバへ通知します...");
        for s in &self.subscrivers {
            s.on_notify(dir);
        }
    }
}

fn main() {
    let mut server = SwipeServer {
        subscrivers: Vec::new(),
    };
    let sub1 = RustSubscriber;
    let sub2 = Rust2Subscriber;

    // mutなので、中のsubscriberrsを書き換えることが可能
    server.add_subscriber(&sub1);
    server.add_subscriber(&sub2);

    server.push_swipe(Direction::Right);
    server.push_swipe(Direction::Left);
}

// int b = a;でaは空っぽになる
// int b = &a;で参照する。aは死なない
