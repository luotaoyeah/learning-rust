/*
   Generic Types, Traits，Lifetimes
       Validating References with Lifetimes
           Thinking in Terms of Lifetimes
 */

pub fn fn_10_03_04_01() {
    println!("-------------------------------------------------- 01");
    {
        /*
           generic lifetime parameter 需要根据函数的具体实现来进行标注；
           如下：函数始终返回参数 x，所以参数 y 可以不用标注 lifetime parameter；
         */

        fn longest<'a>(x: &'a str, y: &str) -> &'a str {
            x
        }
    }

    println!("-------------------------------------------------- 02");
    {
        /*
           如果函数返回一个 reference，
           则该 reference 的 lifetime parameter 必须跟某个参数的 lifetime parameter 一致；
         */

        fn longest<'a>(x: &'a str, y: &str) -> &'a str {
            String::from("hello").as_str() // [E0597]: borrowed value does not live long enough
        }
    }
}
