/*
   Common Collections
       Strings
           Indexing into Strings
               internal Representation

 */

#![allow(unused_variables)]

pub fn fn_08_02_04_02() {
    println!("-------------------------------------------------- 01");
    {
        /*
           Stirng 底层是用 Vec<u8> 实现的，存储的 utf8 字符可能会占两个字节（byte），
           因此通过 index 获取到的可能只是一个 utf8 字符的一部分，是一个非法的字符，
           因此 rust 不允许使用 index 访问 String；
         */

        let string01: String = String::from("З");
        // 单个字符占了两个字节
        println!("{}", string01.len()); // 2
    }
}
