/*
   Programming a Guessing Game
 */

use std::io;

pub fn fn_02() {
    println!("PLEASE INPUT YOUR GUESS:");

    /*
       let 关键字用来声明一个变量；
       rust 中的变量默认是 immutable 的，需要使用 mut 关键字将变量声明为 mutable 的；
       String::new() 中的::表示 new() 方法是 String 类型上的一个 associated function（俗称：静态方法）；
     */
    let mut guess = String::new();

    io::stdin()
        /*
           变量名前面加上 & 表示参数 guess 是一个引用类型；
           引用类型的变量，默认也是 immutable 的，需要在变量名前面加上 mut 来将变量声明为 mutable 的；
         */
        .read_line(&mut guess)
        /*
           read_line() 方法返回一个 io::Result 对象，
           它是一个 enum，有两个 variants：Ok 和 Err；
           它有一个方法 expect()，
               如果当前是 Err，则 expect() 方法会终止程序，打印错误消息；
               如果当前是 Ok，则 expect() 方法会返回 Ok 中的值；
         */
        .expect("FAIL TO READ LINE");

    /*
       println! 的第一个参数中可以使用 {} 作为占位符；
     */
    println!("YOUR GUESS IS {}", guess);
}
