/*
   Enums and Pattern Matching
       The Option Enum
 */

pub fn fn_06_01_03_01() {
    println!("-------------------------------------------------- 01");
    {
        /*
           rust 中没有 null 值，而是通过标准库定义的一个 enum 来描述 null 的概念：Option<T>，
           Option<T> 有两个 variants：Some<T> 和 None，不需要引入，可以直接使用；
         */

        let i = Some(5);
        let s = Some("hello");
        // 当使用 None 时，必须显式标注类型
        let n: Option<i32> = None;
    }

    println!("-------------------------------------------------- 02");
    {
        /*
            使用 Option<T> 描述的值，不能直接使用，因为它们可能为 null，
            因此在使用之前需要判断；
         */

        let i = Some(5);
        if i.is_some() {
            println!("{}", i.unwrap()); // 5
        }
    }
}
