cmake_minimum_required(VERSION 3.5)
project(SwipeServer)

# CapnProtoを探す
find_package(CapnProto REQUIRED)
# swipe.capnpを利用し、C++用のソースコードをビルドする
capnp_generate_cpp(CAPNP_SRCS CAPNP_HDRS swipe.capnp)
# capnp関係もSwipeServerに含める
add_executable(SwipeServer server.cpp ${CAPNP_SRCS})
# 必要なライブラリを紐づける
target_link_libraries(SwipeServer capnp-rpc capnp kj-async kj)