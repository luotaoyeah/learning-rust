/*
   Generic Types, Traits，Lifetimes
       Traits
           Traits as arguments
               Trait Bounds
                   where clauses for clearer code
 */

use std::fmt::Debug;
use std::fmt::Display;

pub fn fn_10_02_03_04() {
    println!("-------------------------------------------------- 01");
    {
        /*
           当 trait bound 很复杂时，可以使用 where 语句；
         */

        fn fn_01<T: Display + Clone, E: Clone + Debug>(t: T, e: E) {}

        fn fn_02<T, E>(t: T, e: E)
        where
            T: Display + Clone,
            E: Clone + Debug,
        {
        }
    }
}
