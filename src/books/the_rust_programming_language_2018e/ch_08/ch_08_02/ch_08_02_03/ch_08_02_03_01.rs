/*
   Common Collections
       Strings
           Upate a String

 */

#![allow(unused_variables)]

pub fn fn_08_02_03_01() {
    println!("-------------------------------------------------- 01");
    {
        /*
           使用 push_str() 附加一个 string slice；
         */

        let mut string01: String = String::from("foo");
        let mut str01: &str = "bar";

        string01.push_str(str01);
        println!("{}", string01); // foobar

        // push_str() 方法参数是一个 string slice，因此不会获取参数的 ownership；
        println!("{}", str01); // bar
    }
}
