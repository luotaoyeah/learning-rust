/*
  Common Programming Concepts
      Data Types
          Scalar Types
              Integer Types
                  _
*/

pub fn fn_03_02_01_01_02() {
    println!("-------------------------------------------------- 01");
    {
        /*
          integer literal 中可以使用 _ 进行分隔，
          增强代码的可读性
        */

        let a: u32 = 100_000;
        println!("{}", a); // 100000
    }
}
