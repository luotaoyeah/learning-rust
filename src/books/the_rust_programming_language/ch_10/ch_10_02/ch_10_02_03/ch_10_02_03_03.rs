/*
   Generic Types, Traits，Lifetimes
       Traits
           Traits as arguments
               Trait Bounds
                   Multiple trait bounds with +
 */

pub fn fn_10_02_03_03() {
    println!("-------------------------------------------------- 01");
    {
        /*
           如果 trait bound 中有多个 trait，使用 + 连接；
         */

        trait A {
            fn fn_a(&self) {
                println!("A.fn_a()");
            }
        }
        trait B {
            fn fn_b(&self) {
                println!("B.fn_b()");
            }
        }

        struct C {}
        impl A for C {}
        impl B for C {}

        fn fn_01<T: A + B>(item: T) {
            item.fn_a();
            item.fn_b();
        }

        fn_01(C {});
    }
}
