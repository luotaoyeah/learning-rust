/*
   使用 extern crate 引入一个外部的 crate；
 */
extern crate communicator;

fn main() {
    /*
       默认情况下，所有代码的访问级别都是 private；
     */
    communicator::client::connect(); // [E0603]: module `client` is private
}
