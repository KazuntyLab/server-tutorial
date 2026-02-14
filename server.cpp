#include <iostream>
#include <vector>
#include <string>

enum class Direction
{
    Left,
    Right
};

class MySubscriber
{
public:
    std::string name;
    MySubscriber(std::string n) : name(n) {}

    void onNotify(Direction dir)
    {
        std::cout << "[Subscriber " << name << "] が受信: "
                  << (dir == Direction::Left ? "左" : "右") << "に動きました！" << std::endl;
    }
};

class SwipeServer
{
private:
    std::vector<MySubscriber *> subscribers;

public:
    // ポインタを渡す
    void addSubscriber(MySubscriber *s)
    {
        subscribers.push_back(s);
        std::cout << "サーバー: " << s->name << " が登録されました。" << std::endl;
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
    MySubscriber sub1("購読者1");
    MySubscriber sub2("購読者2");
    // アドレスを渡す
    // int a = 10;
    // ポインタにはアドレスを渡しましょう
    // int *p = &a;
    server.addSubscriber(&sub1);
    server.addSubscriber(&sub2);
    server.pushSwipe(Direction::Right);
    return 0;
}