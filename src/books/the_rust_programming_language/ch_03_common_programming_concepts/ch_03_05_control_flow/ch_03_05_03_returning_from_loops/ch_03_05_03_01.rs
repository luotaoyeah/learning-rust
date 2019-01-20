/*
  Control Flow
      Repetition with Loops
          Returning from loops
*/

pub fn fn_03_05_03_01() {
    println!("-------------------------------------------------- 01");
    // 如果 break 后面跟上一个表达式，则表达式的值会成为这个 loop 的值

    let mut i: u32 = 0;

    let r = loop {
        if i == 10 {
            break i * i;
        }

        i += 1;
    };

    assert_eq!(r, 100);
}
