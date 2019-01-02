/*
   Generic Types, Traits，Lifetimes
       Traits
           Traits as arguments
               Trait Bounds
 */

pub fn fn_10_02_03_02() {
    println!("-------------------------------------------------- 01");
    {
        /*
           notify(item: impl Trait) 是 trait bound 的语法糖，
           trait bound 是一种泛型约束；
         */

        trait Summary {
            fn summarize(&self) -> String;
        }

        /// 使用 impl Trait 语法糖
        fn notify_01(item: impl Summary) {
            println!("{}", item.summarize());
        }

        /// 使用 trait bound 语法
        fn notify_02<T: Summary>(item: T) {
            println!("{}", item.summarize());
        }
    }
}
