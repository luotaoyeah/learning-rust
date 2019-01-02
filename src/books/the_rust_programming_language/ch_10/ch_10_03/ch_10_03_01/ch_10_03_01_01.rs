/*
   Generic Types, Traits，Lifetimes
       Validating References with Lifetimes
           Preventing Dangling References with Lifetimes
 */

use std::fmt::Display;

pub fn fn_10_03_01_01() {
    println!("-------------------------------------------------- 01");
    {
        /*
           rust 中每一个 reference 都有 lifetime；
           lifetime 的一个主要作用：阻止 danging reference；
         */

        let r;

        {
            let x = 5;
            r = &x;
        }

        println!("{}", r); // [E0597]: `x` does not live long enough
    }
}
