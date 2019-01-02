/*
   Generic Types, Traits，Lifetimes
       Generic Data Types
           Performance of Code Using Generics
 */

pub fn fn_10_01_05_01() {
    println!("-------------------------------------------------- 01");
    {
        /*
           泛型不会影响程序性能，
           rust 在 compile 时期，会将泛型编译为实际类型，
           因此在 runtime 时期，只有实际类型，不会有泛型；
         */
    }
}
