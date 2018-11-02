/*
   Error Handling
       Recoverable Errors with `Result`
           Matching on Different Errors

 */

use std::fs::File;
use std::io::ErrorKind;

pub fn fn_09_02_01_01() {
    println!("-------------------------------------------------- 01");
    {
        let path =
            "src/books/the_rust_programming_language_2018e/ch_09/ch_09_02/ch_09_02_02/foo.txt";
        let f = File::open(path);
        let f = match f {
            Ok(f) => f,
            Err(e) => match e.kind() {
                ErrorKind::NotFound => match File::create(path) {
                    Ok(f) => f,
                    Err(_) => {
                        panic!("fail to create file");
                    }
                },
                _ => panic!("fail to open file"),
            },
        };

        println!("{:?}", f); // File { handle: 0x1e0, path:...
    }
}
