/*
   Error Handling
       Recoverable Errors with `Result`

 */

use std::fs::File;

pub fn fn_09_02_01_01() {
    println!("-------------------------------------------------- 01");
    {
        /*
           Result 是标准库定义的一个 enum，它的定义大致如下：
         */

        enum Result<T, E> {
            Ok(T),
            Err(E),
        }
    }

    println!("-------------------------------------------------- 02");
    {
        let file01 = File::open("foo.txt");
        match file01 {
            Result::Ok(f) => {
                println!("{:?}", f);
            }
            Result::Err(e) => {
                println!("{:?}", e); // Os { code: 2, kind: NotFound, message: "系统找不到指定的文件。" }
            }
        }

        let file02: Result<File, _> = File::open(
            "src/books/the_rust_programming_language_2018e/ch_09/ch_09_02/ch_09_02_01/bar.txt",
        );
        match file02 {
            Ok(f) => {
                println!("{:?}", f); // File { handle: 0x1d4, path...
            }
            Err(e) => {
                println!("{:?}", e);
            }
        }
    }
}
