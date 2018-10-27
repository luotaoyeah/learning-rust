/*
   Understanding Ownership
       References and Borrowing
           Dangling References
 */

pub fn fn_04_02_03_01() {
    println!("-------------------------------------------------- 01");
    {
        /*
           a dangling reference is a reference to an object that no longer exists.
           dangling reference 表示 reference 还存在，但是其引用的数据已经不存在了；
           在 rust 中，编译器会阻止 dangling reference 的产生；
           如果 reference 引用的 value 在 reference 之前离开 scope，编译器会报错；
         */

        let str01 = dangle();
        println!("{}", str01);

        // this function's return type contains a borrowed value,
        // but there is no value for it to be borrowed from
        fn dangle() -> &String {
            let s: String = String::from("hello");

            /*
               此处 s 离开了 scope，因此会调用 drop，
               但是 s 的 reference 会被返回，该 reference 会成为一个 dangling reference，因此报错；
             */
            &s
        }
    }

    println!("-------------------------------------------------- 02");
    {
        let str01: String = dangle();
        println!("{}", str01);

        /*
           正确的做法是直接返回 s，s 发生 move 操作，s 的 ownership 被返回，
           s 不会被 drop；
         */
        fn dangle() -> String {
            let s: String = String::from("hello");
            s
        }
    }
}
