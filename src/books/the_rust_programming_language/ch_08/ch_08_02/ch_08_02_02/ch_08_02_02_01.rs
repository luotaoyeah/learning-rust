/*
   Common Collections
       Strings
           Create a New String

 */

#![allow(unused_variables)]

pub fn fn_08_02_02_01() {
    println!("-------------------------------------------------- 01");
    {
        /*
           跟 Vec<T> 一样，可以使用 String::new() 创建一个新的 String；
         */

        let mut s01: String = String::new();
        s01.push('A');
        s01.push('B');
        s01.push('C');
        println!("{}", s01); // ABC
    }

    println!("-------------------------------------------------- 02");
    {
        /*
           可以使用实现了 Display 的 to_string() 方法创建 String 并设置初始值；
         */

        let s01: String = "ABC".to_string();
        println!("{}", s01); // ABC
    }

    println!("-------------------------------------------------- 03");
    {
        /*
           可以使用 String::from() 方法从 string literal 创建 String；
         */

        let s01: String = String::from("ABC");
        println!("{}", s01); // ABC
    }
}
