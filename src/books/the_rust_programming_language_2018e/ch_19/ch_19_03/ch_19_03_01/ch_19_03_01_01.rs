/*
  Advanced Features
      Advanced Traits
          Specifying Placeholder Types in Trait Definitions with Associated Types
*/

pub fn fn_19_03_01_01() {
    println!("-------------------------------------------------- 01");
    {
        /*
          associated type 有点类似于 generic type，区别在于：
            对于 associated type，因为 trait 不是 generic，
              所以 trait 只能被实现一次，即 associated type 只能够被指定一次；
            而对于 generic type，可以使用不同的实际类型实现多次；
        */

        pub trait Iterator {
            type Item;
            fn next(&mut self) -> Option<Self::Item>;
        }

        struct Counter {}

        /*
          Iterator 只能被 Counter 实现一次，
          因此 associated type 只能够被指定一次；
        */
        impl Iterator for Counter {
            type Item = i32;

            fn next(&mut self) -> Option<Self::Item> {
                unimplemented!()
            }
        }
    }
}
