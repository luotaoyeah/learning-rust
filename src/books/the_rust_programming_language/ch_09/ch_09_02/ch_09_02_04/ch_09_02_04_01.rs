/*
   Error Handling
       Recoverable Errors with `Result`
           Propagating Errors

 */

use std::fs::File;
use std::io;
use std::io::Read;
use std::io::Write;

pub fn fn_09_02_04_01() {
    println!("-------------------------------------------------- 01");
    {
        let name: Result<String, io::Error> = read_username_from_file();
        match name {
            Ok(n) => {
                println!("name01: {}", n);
            }
            Err(e) => {
                println!("error01: {:?}", e);
                match File::create("foo.txt") {
                    Ok(mut f) => match f.write(b"luotao") {
                        Ok(_) => match read_username_from_file() {
                            Ok(n) => {
                                println!("name02: {}", n);
                            }
                            Err(e) => {
                                println!("{:?}", e);
                            }
                        },
                        Err(_e) => {
                            println!("fail to write file");
                        }
                    },
                    Err(_e) => {
                        println!("error02: fail to create file");
                    }
                }
            }
        };
    }
}

/// 从文件中读取用户名
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("foo.txt");

    let mut f = match f {
        Ok(f) => f,
        Err(e) => return Err(e),
    };

    let mut username = String::from("");

    match f.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}
