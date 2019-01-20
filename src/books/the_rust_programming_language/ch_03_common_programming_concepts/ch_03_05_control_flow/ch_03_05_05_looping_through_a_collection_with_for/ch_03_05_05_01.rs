/*
  Control Flow
      Repetition with Loops
          Looping Through a Collection with for
*/

pub fn fn_03_05_05_01() {
    println!("-------------------------------------------------- 01");
    // 使用 while 循环遍历数组

    let arr01: [&str; 3] = ["a", "b", "c"];
    let mut strs: String = String::new();

    let mut i: usize = 0;
    while i < arr01.len() {
        strs.push_str(arr01[i]);
        i += 1;
    }
    println!("{}", strs.as_str()); // abc
}
