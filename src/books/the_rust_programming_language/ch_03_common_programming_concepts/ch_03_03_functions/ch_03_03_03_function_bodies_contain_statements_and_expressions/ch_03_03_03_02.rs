/*
  Functions
      Function Bodies Contain Statements and Expressions
          statement has no return value
*/

pub fn fn_03_03_03_02() {
    println!("-------------------------------------------------- 01");
    // statement 没有返回值，因此不能赋值给其他变量
    // 下面的 let y = 9 是一个 statement，没有返回值，因此不能再赋值给 x
    /*
        let x = (let y = 9); // error: expected expression, found statement (`let`)
    */

    println!("-------------------------------------------------- 02");
    // 单纯的赋值操作是一个 expression，它的值为 ()
    let mut x = 9;
    let y = x = 99;
    println!("{}", x); // 99
    println!("{:?}", y); // ()
}
