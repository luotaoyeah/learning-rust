/*
   Generic Types, Traits，Lifetimes
       Traits
           Traits as arguments
 */

pub fn fn_10_02_03_01() {
    println!("-------------------------------------------------- 01");
    {
        /*
           使用 trait 作为函数参数的类型；
         */

        trait Summary {
            fn summarize(&self) -> String;
        }

        struct A {}
        impl Summary for A {
            fn summarize(&self) -> String {
                String::from("A.summarize()")
            }
        }
        struct B {}
        impl Summary for B {
            fn summarize(&self) -> String {
                String::from("B.summarize()")
            }
        }

        /// 当使用 trait 作为参数的类型时，trait 前面要加上 impl，
        /// 表示参数 item 必须实现 Summary 这个 trait，
        /// 类似于其他语言的'面向接口编程'；
        fn notify(item: impl Summary) {
            println!("notify: {}", item.summarize());
        }

        notify(A {}); // notify: A.summarize()
        notify(B {}); // notify: B.summarize()
    }
}
