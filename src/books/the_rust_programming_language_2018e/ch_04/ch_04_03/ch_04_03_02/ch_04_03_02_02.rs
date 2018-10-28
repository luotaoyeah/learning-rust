/*
   Understanding Ownership
       The Slice Type
           String Slices
 */

pub fn fn_04_03_02_02() {
    println!("-------------------------------------------------- 01");
    {
        /*
           string slice 在内存中会存储两个信息：
               ptr：所引用的数据的地址，
               len：所引用的数据的长度；
         */

        /*
           在 range 中，如果开始索引为 0，或者结束索引为字符串的末尾，
           则它们可以分别省略；
         */

        let s: String = String::from("hello world");
        let hello_slice = &s[..5];
        let world_slice = &s[6..];
        let hello_world_slice = &s[..];
        println!("{} {} {}", hello_slice, world_slice, hello_world_slice); // hello world hello world
    }
}
