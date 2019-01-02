/*
  Advanced Features
      Advanced Types
          Using the Newtype Pattern for Type Safety and Abstraction
          Creating Type Synonyms with Type Aliases
*/

use std::io::Error;
use std::result::Result;

pub fn fn_19_04_01_01() {
    println!("-------------------------------------------------- 01");
    {
        /*
          使用 type 关键字给某个类型声明一个别名（alias）；
        */

        type Meter = i32;

        let len: Meter = 9;

        println!("{}", len / 3); // 3
    }

    println!("-------------------------------------------------- 02");
    {
        type R<T> = Result<T, Error>;

        fn write<T>() -> R<T> {
            Result::Err(Error::from_raw_os_error(2))
        }
    }
}
