/*
  More About Cargo and Crates.io
      Installing Binaries from Crates.io with cargo install
*/

pub fn fn_14_04_01_01() {
    println!("-------------------------------------------------- 01");
    {
        /*
          crates.io 上面的包分成两类：library crate 和 binary crate，
          library crate 是包含 lib.rs 的包，可供其他项目依赖；
          binary crate 是包含 main.rs 的包，可以执行；

          通过 cargo install XXX 命令，可以将 binary crate 安装到本地，
          默认的安装目录为：$HOME/.cargo/bin
        */
    }
}
