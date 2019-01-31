/*
   Understanding Ownership
       References and Borrowing
           Mutable References
 */

pub fn fn_04_02_02_03() {
    println!("-------------------------------------------------- 01");
    {
        /*
            在某个 scope 中，不能同时存在 immutable inference 和 mutable inference，
            但是可以同时存在多个 immutable inference；
         */
        let mut s01: String = String::from("hello");
        let r01 = &s01;
        let r02 = &s01;
        let r03 = &mut s01; // [E0502]: cannot borrow `s01` as mutable because it is also borrowed as immutable
        println!("{} {} {}", r01, r02, r03);
    }
}
