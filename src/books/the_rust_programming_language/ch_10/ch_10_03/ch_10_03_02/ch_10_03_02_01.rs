/*
   Generic Types, Traits，Lifetimes
       Validating References with Lifetimes
           The Borrow Checker
 */

use std::fmt::Display;

pub fn fn_10_03_02_01() {
    println!("-------------------------------------------------- 01");
    {
        /*
           在编译时期，rust 的 borrow checker 会检查 reference 的 lifetime，
           保证其有效性；
         */

        let x = 5;
        let r = &x;
        println!("{}", r);
    }
}
