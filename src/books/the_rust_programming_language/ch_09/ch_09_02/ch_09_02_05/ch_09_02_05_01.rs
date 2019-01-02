/*
   Error Handling
       Recoverable Errors with `Result`
           Propagating Errors
               A Shortcut for Propagating Errors: the `?` Operator

 */

use std::fs;
use std::fs::File;
use std::io;
use std::io::Read;

pub fn fn_09_02_05_01() {
    println!("-------------------------------------------------- 01");
    {
        match read_username_from_file_01() {
            Ok(n) => {
                println!("{}", n);
            }
            Err(e) => {
                println!("{:?}", e);
            }
        }
        match read_username_from_file_02() {
            Ok(n) => {
                println!("{}", n);
            }
            Err(e) => {
                println!("{:?}", e);
            }
        }
        match read_username_from_file_03() {
            Ok(n) => {
                println!("{}", n);
            }
            Err(e) => {
                println!("{:?}", e);
            }
        }
        match read_username_from_file_04() {
            Ok(n) => {
                println!("{}", n);
            }
            Err(e) => {
                println!("{:?}", e);
            }
        }
    }
}

fn read_username_from_file_01() -> Result<String, io::Error> {
    let f = File::open("a.txt");
    let mut f = match f {
        Ok(f) => f,
        Err(e) => return Err(e),
    };

    let mut s: String = String::from("");
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

/*
   在 Result<T, E> 类型后面加上 ?，
   实现：
       如果结果为 Result.Ok，则将 Ok 的值返回；
       如果结果为 Result.Err，则将 Err 的值往上传播；
 */
fn read_username_from_file_02() -> Result<String, io::Error> {
    let mut f = File::open("b.txt")?;
    let mut s = String::from("");
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file_03() -> Result<String, io::Error> {
    let mut s: String = String::from("");
    File::open("c.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file_04() -> Result<String, io::Error> {
    fs::read_to_string("d.txt")
}
