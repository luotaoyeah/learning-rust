/*
  Smart Pointers
      Treating Smart Pointers Like Regular References with the Deref Trait
          Implicit Deref Coercions with Functions and Methods
*/

use std::ops::Deref;

pub fn fn_15_02_02_04() {
    println!("-------------------------------------------------- 01");
    {
        /*
            deref coersion：
                如果函数或者方法的参数是一个：实现了 Deref 特性的类型的一个 reference，如：
                    &MyBox::new(String::from("rust"))
                如果有必要（参数类型不匹配），
                则 rust 会将参数转换为一个：Deref 可以转换的类型的一个 reference，如：
                    &String::from("rust")
                因为 String 也实现了 Deref 特性，因此可以继续转换为：
                    "rust"
                直到匹配参数类型；
        */

        struct MyBox<T>(T);
        impl<T> MyBox<T> {
            fn new(x: T) -> MyBox<T> {
                MyBox(x)
            }
        }
        impl<T> Deref for MyBox<T> {
            type Target = T;

            fn deref(&self) -> &T {
                &self.0
            }
        }

        fn hello(name: &str) {
            println!("{}", name);
        }

        hello("rust");
        hello(&String::from("rust"));
        hello(&MyBox::new(String::from("rust")));
        hello(&(*MyBox::new(String::from("rust")))[..]);
    }
}
