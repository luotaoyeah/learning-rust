/*
   Generic Types, Traits，Lifetimes
       Traits
           Returning Traits
 */

pub fn fn_10_02_04_02() {
    println!("-------------------------------------------------- 01");
    {
        /*
           使用 trait 作为函数的返回类型时，函数不能存在返回不同类型的可能性；
         */

        trait T {}
        struct A {}
        struct B {}
        impl T for A {}
        impl T for B {}

        // [E0308]: if and else have incompatible types
        fn fn_01(b: bool) -> impl T {
            if b {
                A {}
            } else {
                B {}
            }
        }
    }
}
