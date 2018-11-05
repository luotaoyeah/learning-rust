/*
   Understanding Ownership
       References and Borrowing
 */

pub fn fn_04_02_01_02() {
    println!("-------------------------------------------------- 01");
    {
        /*
           使用 reference 而不是直接使用 variable，称之为 borrowing；
         */

        let mut s01: String = String::from("hello");

        change(&s01);

        // 跟 variable 一样，reference 默认也是 immutable 的
        fn change(some_string: &String) -> usize {
            some_string.push_str(" world"); // [E0596]: cannot borrow immutable borrowed content `*some_string` as mutable
            some_string.len()
        }
    }
}
