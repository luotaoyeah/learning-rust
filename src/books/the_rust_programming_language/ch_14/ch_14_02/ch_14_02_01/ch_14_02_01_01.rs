/*
  More About Cargo and Crates.io
      Publishing a Crate to Crates.io
          Making Useful Documentation Comments
          Commonly Used Sections
*/

pub fn fn_14_01_02_01() {
    println!("-------------------------------------------------- 01");
    {
        /*
          使用 cargo doc 命令生成文档，文档位于 /target/doc 目录下面；
          文档注释中常用的一些分块：
              # Examples 示例代码
              # Panics 在哪些场景下可能会发生 panic
              # Errors 如果函数返回一个 Result，则描述可能会返回的 Error
              # Safety 如果函数是 unsafe 的，则描述为什么是 unsafe 的
        
        */
    }
}

/// 加一
///
/// # Examples
///
/// ```
/// let five = 5;
/// let result = add_one(five);
/// ```
///
pub fn add_one(x: i32) -> i32 {
    x + 1
}
