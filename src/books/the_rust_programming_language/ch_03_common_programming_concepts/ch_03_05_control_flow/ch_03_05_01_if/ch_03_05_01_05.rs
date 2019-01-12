/*
  Control Flow
      if Expressions
         Using if in a let Statement
*/

pub fn fn_03_05_01_05() {
    println!("-------------------------------------------------- 01");

    // 当把一个 if expression 赋值给一个变量时，
    // 因为变量最终的值，取决于条件是否成立，
    // 即变量最终的值，可能为任意某个分支的值，
    // 因此每个分支的返回类型必须相同，否则编译报错

    let c: bool = false;
    /*
        let x: i32 = if c { 9 } else { "99" }; // expected i32, found reference
    */
    let x: i32 = if c { 9 } else { 99 };
    println!("{}", x); // 99
}
