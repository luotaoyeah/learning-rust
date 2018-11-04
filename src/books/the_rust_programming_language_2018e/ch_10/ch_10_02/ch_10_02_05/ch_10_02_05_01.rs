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
           当泛型参数满足某个条件时，才实现某个方法；
         */

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
           当泛型参数 T 满足条件：T:PartialOrd 时，才实现方法 cmp_display()；
         */
        impl<T: PartialOrd> Pair<T> {
            fn cmp_display(&self) {
                if self.x > self.y {
                    println!("x > y");
                } else {
                    println!("x < y");
                }
            }
        }

        Pair::new(9, 9).cmp_display();

        struct A {}
        Pair::new(A {}, A {}).cmp_display(); // [E0599]: no method named `cmp_display` found for type `A`
    }
}
