/*
  cargo 是 rust 的构建系统（build system）和包管理器（package manager），
  cargo 完成的工作包括：
      构建代码，
      下载依赖，
      构建依赖

  在 rust 里，依赖包称之为'箱子（crate）'；

  Cargo.lock 文件的作用是：记录当前项目所有依赖确切的版本号；

  cargo build：编译代码，
  cargo run：编译并且运行代码，
  cargo check：检查代码是否编译通过，但是不会生成可执行程序；
 */
