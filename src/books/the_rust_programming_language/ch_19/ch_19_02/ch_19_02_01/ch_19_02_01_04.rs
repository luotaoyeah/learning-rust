/*
  Advanced Features
      Advanced Lifetimes
          The anonymous lifetime
*/

pub fn fn_19_02_01_04() {
    println!("-------------------------------------------------- 01");
    {
        /*
          使用 '_ 生命周期参数表示，此处使用 elided lifetime；
        */
        struct StrWrap<'a>(&'a str);

        fn fn_01<'a>(string: &'a str) -> StrWrap<'a> {
            StrWrap(string)
        }

        fn fn_02(string: &str) -> StrWrap<'_> {
            StrWrap(string)
        }
    }
}
