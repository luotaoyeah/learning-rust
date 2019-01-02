/*
  Printing on the Terminal
      Printing Several Lines of Text
*/

pub fn fn_01_04_02() {
    println!("-------------------------------------------------- 01");
    /*
      println!() 跟 print!() 类似，区别在于：
      会自动在末尾打印一个换行
    */

    /*
      下面两行是等价的
    */
    print!("foo\n");
    println!("foo");
}
