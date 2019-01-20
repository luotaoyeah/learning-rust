/*
  Control Flow
      Repetition with Loops
          Looping Through a Collection with for
*/

use std::ops::Range;

pub fn fn_03_05_05_04() {
    println!("-------------------------------------------------- 01");
    // 使用 for 循环遍历一个数字范围

    let range = Range { start: 1, end: 4 };
    // 3
    // 2
    // 1
    for i in range.rev() {
        println!("{}", i);
    }
}
