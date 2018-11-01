/*
   Common Collections
       Strings
           Upate a String

 */

#![allow(unused_variables)]

pub fn fn_08_02_03_02() {
    println!("-------------------------------------------------- 01");
    {
        /*
           使用 push() 方法附加一个 char；
         */

        let mut string01: String = String::from("foo");
        string01.push('b');
        string01.push('a');
        string01.push('r');

        println!("{}", string01); // foobar
    }
}
