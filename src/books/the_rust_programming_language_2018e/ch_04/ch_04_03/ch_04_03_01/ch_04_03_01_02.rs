/*
   Understanding Ownership
       The Slice Type
 */

pub fn fn_04_03_01_02() {
    println!("-------------------------------------------------- 01");
    {
        /*
           方法一中直接返回索引的问题是：索引没有跟字符串关联，字符串改变之后，索引不会改变；
         */

        /// 查找第一个单词
        fn first_word(s: &String) -> usize {
            let bytes = s.as_bytes();

            for (i, &item) in bytes.iter().enumerate() {
                if item == b' ' {
                    return i;
                }
            }

            s.len()
        }

        let mut s01: String = String::from("hello world");
        let index: usize = first_word(&s01);
        s01.clear();

        // s01 被清空为 "" 之后，index 依然为 5；
        println!("{}", index); // 5
    }
}
