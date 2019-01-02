/*
  Smart Pointers
      Running Code on Cleanup with the Drop Trait
*/

use std::ops::Deref;

pub fn fn_15_03_01_01() {
    println!("-------------------------------------------------- 01");
    {
        /*
          任何类型都可以实现 Drop 特性，实现 Drop 特性时必须实现 drop() 方法；
          当变量离开 scope 时，rust 会调用该变量的 drop() 方法；
        */

        struct CustomSmartPointer {
            data: String,
        }
        impl Drop for CustomSmartPointer {
            fn drop(&mut self) {
                println!("CustomSmartPointer.drop(): {}", self.data);
            }
        }

        let a = CustomSmartPointer {
            data: String::from("a"),
        };
        let b = CustomSmartPointer {
            data: String::from("b"),
        };
        println!("go out of scope");
        /*
          a 和 b 在此处离开 scope，因此会被调用 drop() 方法，
          drop() 方法的调用顺序跟变量的创建顺序相反；
        
          CustomSmartPointer.drop(): b
          CustomSmartPointer.drop(): a
        */
    }
}
