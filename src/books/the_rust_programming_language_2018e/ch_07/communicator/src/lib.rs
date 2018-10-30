/*
   通过 `mod` 关键字来定义一个 module
 */
mod network {
    /*
       module 中的内容需要通过 namespace 的方式来访问，如：network::connect()
     */
    fn connect() {}

    /*
       module 中可以定义 inner module
     */
    mod client {
        fn connect() {}
    }
}

mod client {
    fn connect() {}
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
