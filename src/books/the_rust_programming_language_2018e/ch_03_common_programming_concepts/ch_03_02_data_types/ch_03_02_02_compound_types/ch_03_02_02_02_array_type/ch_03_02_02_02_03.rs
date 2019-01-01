/*
  Common Programming Concepts
      Data Types
          Compound Types
              The Array Type
                  Accessing Array Elements
*/

pub fn fn_03_02_02_02_03() {
    println!("-------------------------------------------------- 01");
    /*
      可以使用索引访问数组元素
    */

    let arr01: [char; 3] = ['a', 'b', 'c'];
    let x: char = arr01[1];
    println!("{}", x); // b
}
