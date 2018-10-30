/*
   将 client 模块的内容移到 client.rs 文件中，
   此处只保留模块声明；
   此处的模块声明可以告诉 lib.rs 去引入 client.rs 的内容；
 */

/*
   当使用 mod 引入一个 module 时，有两种方式去定位这个 module，
   假如 module name 为 foo：
       1. 当前目录下有一个 foo.rs 文件；
       2. 当前目录下有一个 foo 目录，下面有一个 mod.rs 文件；
 */
pub mod client;
mod network;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
