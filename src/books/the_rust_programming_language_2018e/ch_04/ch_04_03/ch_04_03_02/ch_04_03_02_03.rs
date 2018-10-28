/*
   Understanding Ownership
       The Slice Type
           String Slices
 */

pub fn fn_04_03_02_03() {
    println!("-------------------------------------------------- 01");
    {
        /*
           string slice 的类型使用 &str 来表示；
         */

        ///
        fn first_word(s: &String) -> &str {
            let bytes = s.as_bytes();

            for (i, &item) in bytes.iter().enumerate() {
                if item == b' ' {
                    return &s[..i];
                }
            }

            &s[..]
        }

        let mut s: String = String::from("hello world");
        let slice = first_word(&s);
        println!("{}", slice); // hello

        /*
           因为在某个 scope 中，不能同时存在 immutable reference 和 mutable reference，
           上面调用 first_word(&s) 时，使用了 immutable reference：&s，
           而此处使用了 mutable reference：&mut self，所以编译报错；
         */
        s.clear(); // [E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
    }
}
