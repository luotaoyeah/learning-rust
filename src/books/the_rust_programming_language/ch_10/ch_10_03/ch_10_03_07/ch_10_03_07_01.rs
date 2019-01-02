/*
   Generic Types, Traits，Lifetimes
       Validating References with Lifetimes
           Lifetime Annotations in Method Definitions
 */

pub fn fn_10_03_07_01() {
    println!("-------------------------------------------------- 01");
    {
        struct ImportantExcerpt<'a> {
            part: &'a str,
        }

        impl<'a> ImportantExcerpt<'a> {
            fn level(&self) -> i32 {
                3
            }
        }

        impl<'a> ImportantExcerpt<'a> {
            fn announce_and_return_part(&self, announcement: &str) -> &str {
                println!("{}", announcement);
                self.part
            }
        }
    }
}
