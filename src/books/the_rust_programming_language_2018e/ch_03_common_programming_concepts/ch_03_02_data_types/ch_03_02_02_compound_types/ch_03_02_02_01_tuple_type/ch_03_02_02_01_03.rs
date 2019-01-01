/*
  Common Programming Concepts
      Data Types
          Compound Types
              The Tuple Type
                  index
*/

pub fn fn_03_02_02_01_03() {
    println!("-------------------------------------------------- 01");
    /*
      可以使用索引访问 tuple 的各个元素
    */

    let tuple01: (u32, bool, char) = (9, true, 'X');
    let x: u32 = tuple01.0;
    let y: bool = tuple01.1;
    let z: char = tuple01.2;

    println!("{}", x); // 9
    println!("{}", y); // true
    println!("{}", z); // X
}
