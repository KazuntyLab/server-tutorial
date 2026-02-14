#include <iostream>
#include <vector>
#include <string>

enum class Direction
{
    Left,
    Right
};

class SwipeListener
{
public:
    // 純粋仮想関数
    virtual void onNotify(Direction dir) = 0;
    virtual ~SwipeListener() {}
};

class RustSubscriber : public SwipeListener
{
public:
    void onNotify(Direction dir) override
    {
        std::cout << "Rust: "
                  << (dir == Direction::Left ? "左" : "右") << "に動きました！" << std::endl;
    }
};

class Rust2Subscriber : public SwipeListener
{
public:
    void onNotify(Direction dir) override
    {
        std::cout << "Rust2: "
                  << (dir == Direction::Left ? "左" : "右") << "に動きました！" << std::endl;
    }
};

class SwipeServer
{
private:
    std::vector<SwipeListener *> subscribers;

public:
    // ポインタを渡す
    void addSubscriber(SwipeListener *s)
    {
        subscribers.push_back(s);
    }
    void pushSwipe(Direction dir)
    {
        std::cout << "サーバー: スワイプを受信。全サブスクライバへ通知します..." << std::endl;

        for (auto s : subscribers)
        {
            s->onNotify(dir);
        }
    }
};

int main(void)
{
    SwipeServer server;
    RustSubscriber sub1;
    Rust2Subscriber sub2;
    // アドレスを渡す
    // int a = 10;
    // ポインタにはアドレスを渡しましょう
    // int *p = &a;
    server.addSubscriber(&sub1);
    server.addSubscriber(&sub2);
    server.pushSwipe(Direction::Right);
    return 0;
}