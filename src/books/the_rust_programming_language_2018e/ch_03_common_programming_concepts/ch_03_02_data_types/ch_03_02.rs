/*
  Common Programming Concepts
      Data Types
*/

/*
  rust 中每一个值都有对应的数据类型，
  rust 中的数据类型分为两类：scalar（单一的），compound（复合的）；
*/
pub fn fn_03_02_01() {
    /*
      rust 是静态强类型语言，同时也支持类型推断，
      当 rust 无法推断变量的类型时，必须显式声明变量的类型，
      例如：将字符串转换为数值类型时，由于数值类型有很多种（u32，i32，ETC），
           此时必须显式声明变量的类型，否则编译失败；
    */

    println!("-------------------------------------------------- 01");
    /*
        let guess = "42".parse().expect("FAIL TO PARSE"); // [E0282]: type annotations needed
    */

    println!("-------------------------------------------------- 02");
    /*
      显式标注类型
    */
    let x: i32 = "99".parse().expect("FAIL");
    println!("{}", x); // 99

    println!("-------------------------------------------------- 03");
    /*
      使用 turbofish 的形式，即指定泛型参数的实际类型
    */
    let y = "99".parse::<i32>().expect("FAIL");
    println!("{}", y); // 99
}
