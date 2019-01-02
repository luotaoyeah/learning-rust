/*
   Modules
       Referring to Names in Different Modules
           Bring Names into Scope with the `use` Keyword
               Nested Groups in `use` Declarations

 */

/*
   use 语句中可以使用分组和嵌套；
   可以使用 self 表示 module 本身；
   可以使用 * 表示所有 child modules；
 */
use self::foo::{
    bar::{self, b},
    baz::{c, *},
};

mod foo {
    pub mod bar {
        pub const A: i32 = 1;

        pub mod b {
            pub fn fn_01() {
                println!("fn_01()");
            }
        }
    }

    pub mod baz {
        pub const X: i32 = 1;

        pub const Y: i32 = 2;
        pub const Z: i32 = 3;

        pub mod c {
            pub fn fn_02() {
                println!("fn_02()");
            }
        }
    }
}

pub fn fn_07_03_02_03() {
    println!("-------------------------------------------------- 01");
    {
        b::fn_01(); // fn_01()
        c::fn_02(); // fn_02()
        println!("{}", bar::A); // 1
        println!("{}", X); // 1
    }
}
