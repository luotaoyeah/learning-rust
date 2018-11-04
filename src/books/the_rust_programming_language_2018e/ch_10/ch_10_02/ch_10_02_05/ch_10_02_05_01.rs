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
