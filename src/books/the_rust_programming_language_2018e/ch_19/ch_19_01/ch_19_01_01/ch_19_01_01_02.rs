/*
  Advanced Features
      Unsafe Rust
          Calling an Unsafe Function or Method
*/

pub fn fn_19_01_01_02() {
    println!("-------------------------------------------------- 01");
    {
        /*
          使用 unsafe 关键字定义一个 unsafe function，
          调用 unsafe function 必须在 unsafe block 中进行；
        */

        unsafe fn dangerous() {}

        unsafe {
            dangerous();
        }
    }
}
