/*
   Generic Types, Traits，Lifetimes
       Traits
           Default Implementations
 */

pub fn fn_10_02_02_02() {
    println!("-------------------------------------------------- 01");
    {
        /*
           trait 中的方法可以互相调用；
         */

        trait Summary {
            fn summarize_author(&self) -> String;

            fn summarize(&self) -> String {
                format!("Summary.summarize(): {}", self.summarize_author())
            }
        }

        struct Tweet {
            author: String,
            content: String,
        }

        impl Summary for Tweet {
            fn summarize_author(&self) -> String {
                format!("{}", self.author)
            }
        }

        let tweet = Tweet {
            author: String::from("tom"),
            content: String::from("CCC"),
        };

        println!("{}", tweet.summarize()); // Summary.summarize(): tom
    }
}
