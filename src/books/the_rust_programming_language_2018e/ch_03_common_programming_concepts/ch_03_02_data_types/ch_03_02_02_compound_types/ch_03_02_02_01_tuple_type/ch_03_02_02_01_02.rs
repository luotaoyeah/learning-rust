/*
  Common Programming Concepts
      Data Types
          Compound Types
              The Tuple Type
                  destructure
*/

pub fn fn_03_02_02_01_02() {
    println!("-------------------------------------------------- 01");
    /*
      可以使用解构（destructuring）获取 tuple 中的元素
    */

    let tuple01: (u32, bool, char) = (9, true, 'X');
    let (x, y, z) = tuple01;
    println!("{:?}", x); // 9
    println!("{:?}", y); // true
    println!("{:?}", z); // 'X'

    println!("-------------------------------------------------- 02");
    /*
      如果对于 tuple 中的某些元素不感兴趣，
      在解构时，可以对该元素使用 _ 占位符
    */

    let (_, y, _) = tuple01;
    println!("{:?}", y); // true
}
