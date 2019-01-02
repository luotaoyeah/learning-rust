/*
  Printing on the Terminal
      Printing Combinations of Literal Strings
          print!()
*/

pub fn fn_01_03_02() {
    println!("-------------------------------------------------- 01");

    /*
      占位符的数量跟参数的数量必须相同，
      否则编译报错
    */

    /*
        print!("{} {}", "foo"); // error: 2 positional arguments in format string, but there is 1 argument
    */

    /*
        print!("{}", "foo", "bar"); // error: argument never used
    */
}
