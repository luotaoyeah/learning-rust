/*
   Generic Types, Traits，Lifetimes
       Traits
           Default Implementations
 */

pub fn fn_10_02_02_01() {
    println!("-------------------------------------------------- 01");
    {
        /*
           trait 中的方法可以提供默认的实现，
           在实现该 trait 的时候，可以使用默认实现，也可以覆写；
         */

        trait Summary {
            fn summarize(&self) -> String {
                String::from("Summary.summarize()")
            }
        }

        struct Article {
            author: String,
            title: String,
            content: String,
        }

        impl Summary for Article {
            fn summarize(&self) -> String {
                String::from("Article.summarize()")
            }
        }

        let article = Article {
            author: String::from("tom"),
            title: String::from("TTT"),
            content: String::from("CCC"),
        };

        println!("{}", article.summarize()); // Article.summarize()

        struct Tweet {
            username: String,
            content: String,
        }

        impl Summary for Tweet {}

        let tweet = Tweet {
            username: String::from("cat"),
            content: String::from("CC"),
        };

        println!("{}", tweet.summarize()); // Summary.summarize()
    }
}
