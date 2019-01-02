/*
  Patterns and Matching
      Pattern Syntax
          Ignoring Values in a Pattern
              Ignoring an Unused Variable by Starting Its Name with _
*/

pub fn fn_18_03_03_03() {
    println!("-------------------------------------------------- 01");
    {
        /*
          如果定义了变量却没有使用，compiler 会警告，
          此时可以设置变量名为 _ 开头，则 compiler 不会再警告；
        */

        let _x = 0;
        let y = 0; // warning: unused variable: `y`
    }

    println!("-------------------------------------------------- 02");
    {
        /*
          使用 _ 开头的变量，在匹配时仍然会绑定到值；
        */

        let s = Some(String::from("hello"));
        if let Some(_s) = s {
            /*
              因为 _s 会绑定到匹配的值，
              因此 s 的 ownership 会 move 到 _s 中；
            */
            println!("{:?}", _s);
        }

        println!("{:?}", s); // [E0382]: use of partially moved value: `s`
    }
}
