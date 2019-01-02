/*
  Advanced Features
      Advanced Lifetimes
*/

pub fn fn_19_02_01_01() {
    println!("-------------------------------------------------- 01");
    {
        struct Context<'s>(&'s str);

        /*
          's: 'c 表示 s 表示的 lifetime 要大于等于 c 表示的 lifetime；
        */
        struct Parser<'c, 's: 'c> {
            context: &'c Context<'s>,
        }

        impl<'c, 's> Parser<'c, 's> {
            fn parse(&self) -> Result<(), &'s str> {
                Err(&self.context.0[1..])
            }
        }

        fn parse_context(context: Context) -> Result<(), &str> {
            Parser { context: &context }.parse()
        }
    }
}
