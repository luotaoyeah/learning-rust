/*
   Understanding Ownership
       The Slice Type
           String Slices
               String Literals Are Slices
 */

pub fn fn_04_03_03_01() {
    println!("-------------------------------------------------- 01");
    {
        /*
           string literal 是一个 string slice；
           因为 string literal 是硬编码到程序代码中的，
           所以 string literal 是一个 string slice，指向的数据是程序代码的一部分；
         */

        let s: &str = "hello world";
    }
}
