/*
  Smart Pointers
      Dropping a Value Early with core::mem::drop
*/

use std::ops::Deref;

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("CustomSmartPointer.drop(): {}", self.data);
    }
}

pub fn fn_15_03_02_01() {
    println!("-------------------------------------------------- 01");
    {
        let mut a = CustomSmartPointer {
            data: String::from("a"),
        };

        /*
                // Drop 特性中的 drop() 方法，不能手动调用，否则编译报错
                a.drop(); // [E0040]: explicit use of destructor method
        */
        println!("go out out scope");
    }

    println!("-------------------------------------------------- 02");
    {
        /*
          如果想提前调用 drop() 方法释放资源，需要调用 core::mem::drop() 方法；
        */

        let a = CustomSmartPointer {
            data: String::from("a"),
        };

        core::mem::drop(a); // CustomSmartPointer.drop(): a
        println!("go out of scope");
    }
}
