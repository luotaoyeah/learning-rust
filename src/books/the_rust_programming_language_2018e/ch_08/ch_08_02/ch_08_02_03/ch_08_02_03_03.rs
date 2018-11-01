/*
   Common Collections
       Strings
           Upate a String
               + operator

 */

#![allow(unused_variables)]

pub fn fn_08_02_03_03() {
    println!("-------------------------------------------------- 01");
    {
        /*
           可以使用 + 拼接多个 String；
         */

        let string01: String = String::from("foo");
        let string02: String = String::from("bar");

        /*
           使用 + 拼接 String 的时候，相当于调用了一个 add() 方法，
           方法的签名为：fn add(self, &str) -> String，
               第一个参数为 + 左边的 String，它发生了 move 操作；
               第二个参数为 + 右边的 string slice；

           这里第二个参数期望的是 &str，而我们传递的是 &String，
           因此这里发生了强制类型转换（coercion），将 &string02 转换为 &string02[..]；
         */
        let string03: String = string01 + &string02;
        println!("{}", string03);
        println!("{}", string01); // [E0382]: use of moved value: `string01`
    }
}
