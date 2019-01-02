/*
   Generic Types, Traits，Lifetimes
       Traits
           Using Trait Bounds to Conditionally Implement Methods
 */

use std::fmt::Display;

pub fn fn_10_02_05_01() {
    println!("-------------------------------------------------- 01");
    {
        /*
           当泛型参数满足某个条件时，才实现某个 trait；
         */

        trait MyTrait {
            fn fn_01();
        }

        /*
           当泛型参数 T 满足条件：T:PartialOrd 时，才实现 trait，
           即：当泛型参数 T 已经实现了某个 trait（PartialOrd）时，才为它实现新的 trait（MyTrait）；
           这个新的实现称之为：blanket implementation；
         */
        impl<T: PartialOrd> MyTrait for T {
            fn fn_01() {
                println!("fn_01()");
            }
        }
    }
}
