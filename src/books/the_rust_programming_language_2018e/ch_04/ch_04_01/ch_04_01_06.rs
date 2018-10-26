/*
   Understanding Ownership
       Memory and Allocation
           Ways Variables and Data Interact: Clone
 */

pub fn fn_04_01_06() {
    println!("-------------------------------------------------- 01");
    {
        /*
           如果确实想复制 heap 上的实际数据，而不仅仅是 stack 上的元数据，
           可以使用 clone() 方法；
         */

        let s01 = String::from("hello");
        let s02 = s01.clone();

        println!("{} {}", s01, s02);
    }
}
