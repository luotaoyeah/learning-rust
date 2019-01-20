/*
  Control Flow
      Repetition with Loops
          Conditional Loops with while
*/

pub fn fn_03_05_04_01() {
    println!("-------------------------------------------------- 01");

    let mut i: u32 = 3;

    while i > 0 {
        println!("{}", i);
        i -= 1;
    }

    println!("-------------------------------------------------- 02");
    // 使用 loop 循环模拟 while 循环
    let mut j: u32 = 3;

    loop {
        if j <= 0 {
            break;
        }

        println!("{}", j);
        j -= 1;
    }
}
