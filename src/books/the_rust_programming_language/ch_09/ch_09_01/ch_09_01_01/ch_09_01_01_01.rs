/*
   Error Handling
       Unrecoverable Errors with `panic!`

 */

pub fn fn_09_01_01_01() {
    println!("-------------------------------------------------- 01");
    {
        /*
           rust 中的 error 分成两类：recoverable 和 unrecoverable；
           rust 中没有 exception；
           当遇到 recoverable error 时，可以执行 panic! 宏终止程序运行；
         */

        panic!("crash and burn");
    }
}
