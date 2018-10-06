/*
   Programming a Guessing Game
 */

extern crate rand;

use self::rand::Rng;
use std::io;

pub fn fn_02() {
    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("ANSWER: {}", secret_number);

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

    /*
       Cargo.lock 文件记录了所有依赖的 crates 的确切版本，保证了每次构建都可重现；
       除非手动更新依赖，否则所有依赖的版本都会保持不变；

       cargo update 命令用于自动更新所有依赖到最新的 patch 版本（major.minor.patch），
       并将新的版本号保存到 Cargo.lock 文件；

       如果要更新 minor 或者 major 版本号，需要手动修改 Cargo.toml 中的版本号，
       并通过 cargo build 命令更新 Cargo.lock 中对应的版本号；
     */
}
