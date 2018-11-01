/*
   Common Collections
       Strings
           What Is a String?

 */

#![allow(unused_variables)]
pub fn fn_08_01_06_01() {
    println!("-------------------------------------------------- 01");
    {
        /*
           rust 的语言核心只提供了一种 string 类型，即 str，即 string slice，
           通常使用它的 borrow 形式，即 &str，string literal 就是一种 string slice；
         */

        let str01: &str = "foo";

        let str02: String = String::from("foo");

        let str03: &str = &str02[..=2];
    }

    println!("-------------------------------------------------- 02");
    {
        /*
          String 类型是 rust 标准库提供的一种 string 类型；
        */

        let str01: String = String::from("foo");
    }
}
