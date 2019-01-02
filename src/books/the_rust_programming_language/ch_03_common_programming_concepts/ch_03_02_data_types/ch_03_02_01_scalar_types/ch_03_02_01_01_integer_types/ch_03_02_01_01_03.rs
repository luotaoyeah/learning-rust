/*
  Common Programming Concepts
      Data Types
          Scalar Types
              Integer Types
                  suffix
*/

pub fn fn_03_02_01_01_03() {
    println!("-------------------------------------------------- 01");
    {
        /*
          integer literal 中可以使用类型作为后缀
        */

        let a: u8 = 9u8;
        let b: u16 = 0xfu16;
        let c: u32 = 0o7u32;
        let d: u64 = 0b1u64;

        println!("{}, {}, {}, {}", a, b, c, d); // 9, 15, 7, 1
    }

    println!("-------------------------------------------------- 02");
    {
        /*
          byte literal 中不能使用类型作为后缀
        */

        /*
                let a:u8=b'A'u8; // error: byte literal with a suffix is invalid
        */
    }
}
