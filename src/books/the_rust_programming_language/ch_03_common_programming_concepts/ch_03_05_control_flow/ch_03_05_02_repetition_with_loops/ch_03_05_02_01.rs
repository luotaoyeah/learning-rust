/*
  Control Flow
      Repetition with Loops
          Repeating Code with loop
*/

pub fn fn_03_05_02_01() {
    println!("-------------------------------------------------- 01");
    // 使用 loop 关键字定义一个 loop block 代码块，
    // 代码块中的代码会不断循环执行，除非使用 break 显式地终止循环

    let mut i: u32 = 0;

    // 1
    // 2
    // 3
    loop {
        if i >= 3 {
            break;
        }

        i += 1;
        println!("{}", i);
    }
}
