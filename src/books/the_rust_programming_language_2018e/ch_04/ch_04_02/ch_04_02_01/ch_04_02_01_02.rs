/*
   Understanding Ownership
       References and Borrowing
 */

pub fn fn_04_02_01_02() {
    println!("-------------------------------------------------- 01");
    {
        /*
           使用 reference 作为函数参数，称之为 borrowing；
           跟 variable 一样，reference 默认也是 immutable 的；
         */

        let s01: String = String::from("hello");

        get_length(&s01);

        ///
        fn get_length(s: &String) -> usize {
            s.push_str(" world"); // [E0596]: cannot borrow immutable borrowed content `*s` as mutable
            s.len()
        }
    }
}
