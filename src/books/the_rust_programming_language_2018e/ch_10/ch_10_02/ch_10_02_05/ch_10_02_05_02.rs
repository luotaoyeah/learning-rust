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

        struct Pair<T> {
            x: T,
            y: T,
        }

        impl<T> Pair<T> {
            fn new(x: T, y: T) -> Pair<T> {
                Pair { x, y }
            }
        }

        /*
           当泛型参数 T 满足条件：T:PartialOrd 时，才实现 trait；
         */
        impl<T: PartialOrd> MyTrait for Pair<T> {
            fn fn_01() {
                println!("fn_01()");
            }
        }
    }
}
