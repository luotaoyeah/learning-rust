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
           string literal 也是一个 string slice；
         */

        let s: &str = "hello world";
    }
}
