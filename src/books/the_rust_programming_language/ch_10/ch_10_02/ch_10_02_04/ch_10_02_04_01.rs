/*
   Generic Types, Traits，Lifetimes
       Traits
           Returning Traits
 */

use std::fmt::Debug;
use std::fmt::Display;

pub fn fn_10_02_04_01() {
    println!("-------------------------------------------------- 01");
    {
        /*
           可以将 trait 作为函数的返回类型，语法跟将 trait 作为函数的参数类型一样；
         */

        trait A {
            fn fn_a(&self) {
                println!("A.fn_a()");
            }
        }

        struct S {}
        impl A for S {}

        fn fn_01() -> impl A {
            S {}
        }

        fn_01().fn_a(); // A.fn_a()
    }
}
