/*
   Generic Types, Traits，Lifetimes
       Validating References with Lifetimes
           Lifetime Annotations in Struct Definitions
 */

pub fn fn_10_03_05_01() {
    println!("-------------------------------------------------- 01");
    {
        /*
           如果 struct 的 field 是一个 reference，
           则该 struct 必须声明 lifetime parameter；
         */

        struct ImportantExcerpt<'a> {
            part: &'a str,
        }

        {
            let text = String::from("hello world");

            let excerpt = ImportantExcerpt { part: &text[..] };
            println!("{}", excerpt.part); // hello world
        }
    }
}
