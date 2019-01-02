/*
  Common Programming Concepts
      Differences Between Variables and Constants
*/

/*
  变量（variable）和常量（constant）的区别：
      1. 变量默认是 immutable 的，但是可以通过 mut 声明为 mutable 的，
         常量始终是 immutable 的；
      2. 变量使用 let 关键字声明，
         常量使用 const 关键字声明；
      3. 常量必须显式声明类型；
      4. 赋给常量的值必须是常量值，不能是函数调用等需要在 runtime 进行计算的值；
*/

/*
  数字中间可以添加下划线（_）增加可读性；
*/
const MAX_POINTS: u32 = 100_000;

pub fn fn_03_01_02() {
    println!("{}", MAX_POINTS / 100);
}
