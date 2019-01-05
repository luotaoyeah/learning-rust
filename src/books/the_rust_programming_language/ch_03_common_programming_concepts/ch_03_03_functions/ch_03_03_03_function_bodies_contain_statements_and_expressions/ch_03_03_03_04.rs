/*
  Functions
      Function Bodies Contain Statements and Expressions
          block is an expression
*/

pub fn fn_03_03_03_04() {
    println!("-------------------------------------------------- 01");
    // 代码块 {} 也是一个 expression

    // 下面的代码块没有返回语句，因此它的值为 ()
    let x01 = {
        let y = 9;
    };
    println!("{:?}", x01); // ()

    println!("-------------------------------------------------- 02");
    // 下面的代码块，最后一行有一个 expression（没有分号结尾）
    // 因此它的值就是最后那行的 expression 的值
    let x02 = {
        let y = 9;
        y * 9
    };
    println!("{:?}", x02); // 81
}
