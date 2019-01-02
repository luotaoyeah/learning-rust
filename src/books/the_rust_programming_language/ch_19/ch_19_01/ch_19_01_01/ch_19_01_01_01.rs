/*
  Advanced Features
      Unsafe Rust
          Unsafe Superpowers
              Dereferencing a Raw Pointer
*/

pub fn fn_19_01_01_01() {
    println!("-------------------------------------------------- 01");
    {
        /*
          使用 unsafe 关键字定义 unsafe 代码，
          在 unsafe 代码块中可以进行四种操作，称之为 unsafe superpowers：
              dereference a raw pointer
              调用 unsafe 函数
              实现 unsafe 特性
              访问或者修改 mutable static variable
        
          unsafe 代码的范围应该尽可能小，方便查错；
        */

        /*
          raw pointer 是 unsafe rust 中的类型，跟 reference 类似，区别在于：
              row pointer 不受 borrow rules 约束，可以同时存在 immutable pointer 和 mutable pointer；
              不能保证指向的是有效的内存；
              允许为空；
              没有实现自动的 cleanup；
        
          immutable raw pointer 形式为：*const T，
          mutable raw pointer 形式为：*mut T；
        */

        /*
          从 reference 创建一个 raw pointer；
          可以在 safe rust 中创建 raw pointer，
          但是只能在 unsafe rust 中 dereference raw pointer；
        */

        let mut num = 5;

        let p01 = &num as *const i32;
        let p02 = &mut num as *mut i32;

        unsafe {
            println!("{}", *p01); // 5
            println!("{}", *p02); // 5
        }
    }

    println!("-------------------------------------------------- 02");
    {
        /*
          可以从任意的内存地址创建一个 raw pointer
        */

        let address = 0x012345usize;

        let p = address as *const i32;
    }
}
