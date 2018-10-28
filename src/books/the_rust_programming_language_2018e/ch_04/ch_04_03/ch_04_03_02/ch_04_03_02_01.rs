/*
   Understanding Ownership
       The Slice Type
           String Slices
 */

pub fn fn_04_03_02_01() {
    println!("-------------------------------------------------- 01");
    {
        /*
           string slice 是一个 reference，跟普通的 reference 的区别是：
           它只引用了部分数据；
         */

        let s01: String = String::from("hello world");

        /*
           后面的 [0..5] 是一个 range，表示引用的数据范围，左闭右开；
         */
        let hello_slice = &s01[0..5];
        let world_slice = &s01[6..11];

        println!("{} {}", hello_slice, world_slice); // hello world
    }

    println!("-------------------------------------------------- 02");
    {
        /*
           可以使用 [n..=m] 的形式，表示左闭右闭；
         */

        let s: String = String::from("hello world");
        let hello_slice = &s[0..=4];
        let world_slice = &s[6..=10];
        println!("{} {}", hello_slice, world_slice); // hello world
    }
}
