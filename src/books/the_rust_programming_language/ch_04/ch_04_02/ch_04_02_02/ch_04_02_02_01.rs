/*
   Understanding Ownership
       References and Borrowing
           Mutable References
 */

pub fn fn_04_02_02_01() {
    println!("-------------------------------------------------- 01");
    {
        /*
           通过 &mut 声明一个 mutable 的 reference；
         */

        // 将 variable 声明为 mutable
        let mut s01: String = String::from("hello");

        // mutable 的 reference 在传递的时候，需要使用 &mut 标注；
        let len = change(&mut s01);
        println!("{} {}", s01, len); // hello world 11

        // mutable 的 reference 在声明的时候，需要使用 &mut 标注；
        fn change(some_string: &mut String) -> usize {
            some_string.push_str(" world");
            some_string.len()
        }
    }
}
