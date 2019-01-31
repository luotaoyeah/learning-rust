/*
  Naming Objects
      3.8. Type Consistency in Assignments
*/

pub fn fn_03_08_01() {
    println!("-------------------------------------------------- 01");
    // 给变量赋值时，必须是相同类型的数据，否则编译报错

    let mut n: u32 = 9;
    /*
        n = -9; // error[E0600]: cannot apply unary operator `-` to type `u32`
    */
    println!("{}", n);
}
