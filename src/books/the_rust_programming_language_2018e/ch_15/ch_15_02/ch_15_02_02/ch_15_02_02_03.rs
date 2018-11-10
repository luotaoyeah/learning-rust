/*
  Smart Pointers
      Treating Smart Pointers Like Regular References with the Deref Trait
          Treating a Type Like a Reference by Implementing the Deref Trait
*/

use std::ops::Deref;

pub fn fn_15_02_02_03() {
    println!("-------------------------------------------------- 01");
    {
        struct MyBox<T>(T);
        impl<T> MyBox<T> {
            fn new(x: T) -> MyBox<T> {
                MyBox(x)
            }
        }

        impl<T> Deref for MyBox<T> {
            type Target = T;

            /// 实现 deref() 方法，返回一个 reference
            fn deref(&self) -> &T {
                &self.0
            }
        }

        let x = 5;
        let y = MyBox::new(x);
        // 此处的 *y 相当于 *(y.deref())
        assert_eq!(5, *y);
    }
}
