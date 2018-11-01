/*
   Common Collections
       Strings
           Slicing Strings

 */

#![allow(unused_variables)]

pub fn fn_08_02_05_01() {
    println!("-------------------------------------------------- 01");
    {
        /*
           可以通过指定 range 获取 String 的 string slice；
         */

        let string01: String = String::from("Здр");
        let str01: &str = &string01[0..4];
        println!("{}", str01); // Зд
    }

    println!("-------------------------------------------------- 02");
    {
        /*
           如果 range 的范围刚好包含了某个字符的一部分，运行报错（panic）；
         */

        let string01: String = String::from("З");
        let str01: &str = &string01[0..1];
        // thread 'main' panicked at 'byte index 1 is not a char boundary; it is inside 'З' (bytes 0..2) of `З`'
        println!("{}", str01);
    }
}
