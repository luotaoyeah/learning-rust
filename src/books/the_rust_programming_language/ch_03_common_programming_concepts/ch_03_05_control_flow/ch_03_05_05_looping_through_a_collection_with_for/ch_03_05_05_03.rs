/*
  Control Flow
      Repetition with Loops
          Looping Through a Collection with for
*/

pub fn fn_03_05_05_03() {
    println!("-------------------------------------------------- 01");
    // 使用 for 循环遍历数组

    let arr01: [&str; 3] = ["a", "b", "c"];
    let mut strs: String = String::new();

    for el in arr01.iter() {
        strs.push_str(el);
    }

    println!("{}", strs); // abc
}
