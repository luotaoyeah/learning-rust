/*
   Error Handling
       Recoverable Errors with `Result`
           Shortcut for Panic on Error: `unwrap()`，`expect()`

 */

use std::fs::File;

pub fn fn_09_02_03_01() {
    println!("-------------------------------------------------- 01");
    {
        /*
           如果结果为 Result::Ok，则 unwrap() 方法返回 Ok 的值，
           否则 panic；
         */
        // thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "系统找不到指定的文件。" }
        let f = File::open("foo.txt").unwrap();
    }

    println!("-------------------------------------------------- 02");
    {
        /*
           expect() 方法跟 unwrap() 方法类似，但是可以指定错误信息；
         */
        // thread 'main' panicked at 'fail to open foo.txt: Os { code: 2, kind: NotFound, message: "系统找不到指定的文件。" }
        let f = File::open("foo.txt").expect("fail to open foo.txt");
    }
}
