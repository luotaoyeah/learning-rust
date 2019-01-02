/*
  Printing on the Terminal
      Printing Several Lines of Text
*/

pub fn fn_01_04_01() {
    println!("-------------------------------------------------- 01");
    /*
      使用 print!() 打印多行文本：
      当字符串中包含 \n 时，它会被解析为换行
    */

    // foo
    // bar
    // baz
    print!("foo\nbar\nbaz");
}
