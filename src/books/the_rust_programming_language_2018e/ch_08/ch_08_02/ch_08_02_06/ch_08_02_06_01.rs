/*
   Common Collections
       Strings
           Iterating over Strings

 */

#![allow(unused_variables)]

pub fn fn_08_02_06_01() {
    println!("-------------------------------------------------- 01");
    {
        let string01: String = String::from("नमस्ते");

        /*
            可以通过 bytes() 方法获取 String 的 bytes；
         */
        for b in string01.bytes() {
            println!("{}", b);
        }

        /*
           可以通过 chars() 方法获取 String 的 chars;
         */
        for c in string01.chars() {
            println!("{}", c);
        }
    }
}
