/*
   Understanding Ownership
       The Slice Type
           String Slices
               String Slices as Parameters
 */

pub fn fn_04_03_04_01() {
    println!("-------------------------------------------------- 01");
    {
        /*
           将参数的类型由 &String 改为 &str，即由 string reference 改为 string slice；
           因为 &String 可以由 &str 来表示，如 &s 表示为 &s[..]；
         */

        fn first_word(s: &str) -> &str {
            let bytes = s.as_bytes();

            for (i, &item) in bytes.iter().enumerate() {
                if item == b' ' {
                    return &s[..i];
                }
            }

            s
        }

        let s01: String = String::from("hello world");
        println!("{}", first_word(&s01[..])); // hello

        let s02: &str = "hello world";
        println!("{}", first_word(s02)); // hello
    }
}
