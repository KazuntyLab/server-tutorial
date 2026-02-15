#include <iostream>
#include <vector>
#include <capnp/capability.h>
#include <capnp/rpc-twoparty.h>
#include <kj/async-io.h>
#include "swipe.capnp.h"

class SwipeServiceImpl final : public SwipeService::Server
{
public:
    kj::Promise<void> pushSwipe(PushSwipeContext context) override
    {
        auto direction = context.getParams().getEvent().getDirection();

        std::cout << "[Server] スワイプを受信: "
                  << (direction == Direction::LEFT ? "LEFT" : "RIGHT") << std::endl;

        for (auto &listener : listeners)
        {
            auto request = listener.onSwipeRequest();
            auto event = request.initEvent();
            event.setDirection(direction);
            request.send().detach([](kj::Exception &&e)
                                  { std::cerr << "通知エラー: " << e.getDescription().cStr() << std::endl; });
        }

        return kj::READY_NOW;
    }

    kj::Promise<void> subscribe(SubscribeContext context) override
    {
        auto listener = context.getParams().getListener();
        listeners.push_back(std::move(listener));

        std::cout << "[Server] 新しいサブスクライバが登録されました！" << std::endl;
        return kj::READY_NOW;
    }

private:
    std::vector<SwipeListener::Client> listeners;
};

int main()
{
    kj::AsyncIoContext ioContext = kj::setupAsyncIo();
    auto &waitScope = ioContext.waitScope;
    auto &network = ioContext.provider->getNetwork();

    auto address = network.parseAddress("0.0.0.0", 9000).wait(waitScope);
    auto listener = address->listen();

    auto service = kj::heap<SwipeServiceImpl>();

    capnp::TwoPartyServer rpcServer(kj::mv(service));

    std::cout << "====================================" << std::endl;
    std::cout << "  Swipe RPC Server 起動完了 (Port: 9000)" << std::endl;
    std::cout << "====================================" << std::endl;

    while (true)
    {
        auto stream = listener->accept().wait(waitScope);
        rpcServer.accept(kj::mv(stream));
        std::cout << "[Server] クライアントが接続しました。" << std::endl;
    }
    return 0;
}

// #include <iostream>
// #include <vector>
// #include <string>

// enum class Direction
// {
//     Left,
//     Right
// };

// class SwipeListener
// {
// public:
//     // 純粋仮想関数
//     virtual void onNotify(Direction dir) = 0;
//     virtual ~SwipeListener() {}
// };

// class RustSubscriber : public SwipeListener
// {
// public:
//     void onNotify(Direction dir) override
//     {
//         std::cout << "Rust: "
//                   << (dir == Direction::Left ? "左" : "右") << "に動きました！" << std::endl;
//     }
// };

// class Rust2Subscriber : public SwipeListener
// {
// public:
//     void onNotify(Direction dir) override
//     {
//         std::cout << "Rust2: "
//                   << (dir == Direction::Left ? "左" : "右") << "に動きました！" << std::endl;
//     }
// };

// class SwipeServer
// {
// private:
//     std::vector<SwipeListener *> subscribers;

// public:
//     // ポインタを渡す
//     void addSubscriber(SwipeListener *s)
//     {
//         subscribers.push_back(s);
//     }
//     void pushSwipe(Direction dir)
//     {
//         std::cout << "サーバー: スワイプを受信。全サブスクライバへ通知します..." << std::endl;

//         for (auto s : subscribers)
//         {
//             s->onNotify(dir);
//         }
//     }
// };

// int main(void)
// {
//     SwipeServer server;
//     RustSubscriber sub1;
//     Rust2Subscriber sub2;
//     // アドレスを渡す
//     // int a = 10;
//     // ポインタにはアドレスを渡しましょう
//     // int *p = &a;
//     server.addSubscriber(&sub1);
//     server.addSubscriber(&sub2);
//     server.pushSwipe(Direction::Right);
//     return 0;
// }