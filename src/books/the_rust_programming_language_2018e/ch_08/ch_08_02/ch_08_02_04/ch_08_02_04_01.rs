/*
   Common Collections
       Strings
           Indexing into Strings

 */

#![allow(unused_variables)]

pub fn fn_08_02_04_01() {
    println!("-------------------------------------------------- 01");
    {
        /*
           String 不支持通过 index 下标访问；
         */

        let string01: String = String::from("foo");
        println!("{}", string01[0]); // [E0277]: the type `std::string::String` cannot be indexed by `{integer}`
    }
}
