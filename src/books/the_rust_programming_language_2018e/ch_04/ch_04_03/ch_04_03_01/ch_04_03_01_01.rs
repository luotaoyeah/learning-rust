/*
   Understanding Ownership
       The Slice Type
 */

pub fn fn_04_03_01_01() {
    println!("-------------------------------------------------- 01");
    {
        /*
           问题：
               给定一个字符串，查找并返回字符串中的第一个单词；
               如果字符串本身就是一个单词，则返回字符串本身；
           方法一：
               返回第一个单词末尾的索引，如：给定 "hello world"，返回 5；
         */

        fn first_word(s: &String) -> usize {
            // 使用 as_bytes() 方法将 s 转换为字节数组（byte array）
            let bytes = s.as_bytes();

            for (i, &item) in bytes.iter().enumerate() {
                if item == b' ' {
                    return i;
                }
            }

            s.len()
        }

        let s01: String = String::from("hello world");
        println!("{}", first_word(&s01)); // 5

        let s02: String = String::from("rust");
        println!("{}", first_word(&s02)); // 4
    }
}
