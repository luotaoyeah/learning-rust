/*
  Naming Objects
      3.5. The Leading Underscore
          _
*/

pub fn fn_03_05_02() {
    println!("-------------------------------------------------- 01");
    // 如果变量名仅仅是一个 _，那么它表示的是一个 placeholder，而不是一个 variable，
    // 因此不能在其他地方被使用

    let _: u32 = 9;
    /*
        println!("{}", _); // error: expected expression, found reserved identifier `_`
    */
}
