/*
   Error Handling
       Unrecoverable Errors with `panic!`
           Using a `panic!` Backtrace

 */

pub fn fn_09_01_01_02() {
    println!("-------------------------------------------------- 01");
    {
        let vec: Vec<i32> = vec![1, 2, 3];
        vec[99];
    }
}
