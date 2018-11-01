/*
   Common Collections
       Strings
           Upate a String
               format!

 */

#![allow(unused_variables)]

pub fn fn_08_02_03_04() {
    println!("-------------------------------------------------- 01");
    {
        /*
           如果需要拼接多个字符串，可以使用 format! 宏，
           format! 和 println! 的原理是一样的；
         */

        let string01: String = String::from("tic");
        let string02: String = String::from("tac");
        let string03: String = String::from("toe");

        let string04: String = string01.clone() + "-" + &string02 + "-" + &string03;
        let string05: String = format!("{}-{}-{}", string01, string02, string03);

        println!("{}", string04); // tic-tac-toe
        println!("{}", string05); // tic-tac-toe
    }
}
