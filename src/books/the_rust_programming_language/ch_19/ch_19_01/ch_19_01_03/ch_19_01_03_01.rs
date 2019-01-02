/*
  Advanced Features
      Unsafe Rust
          Implementing an Unsafe Trait
*/

pub fn fn_19_01_03_01() {
    println!("-------------------------------------------------- 01");
    {
        /*
          使用 unsafe 标注一个 unsafe trait 及其实现；
        */

        unsafe trait Foo {}
        unsafe impl Foo for i32 {}
    }
}
