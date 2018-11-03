/*
   Generic Types, Traits，Lifetimes
       Traits
           Defining a Trait
           Implementing a Trait on a Type
 */

pub fn fn_10_02_01_01() {
    println!("-------------------------------------------------- 01");
    {
        /*
           trait 类似其他语言中的 interface；
         */

        pub trait Summary {
            fn summarize(&self) -> String;
        }

        struct Article {
            title: String,
            content: String,
            author: String,
            location: String,
        }

        impl Summary for Article {
            fn summarize(&self) -> String {
                format!("{}, by {} {}", self.title, self.author, self.location)
            }
        }

        let article: Article = Article {
            author: String::from("tom"),
            content: String::from("CCCC"),
            title: String::from("TTTT"),
            location: String::from("CHINA"),
        };

        println!("{}", article.summarize()); // TTTT, by tom CHINA

        struct Tweet {
            username: String,
            content: String,
            reply: bool,
            retweet: bool,
        }

        impl Summary for Tweet {
            fn summarize(&self) -> String {
                format!("{}: {}", self.username, self.content)
            }
        }

        let tweet: Tweet = Tweet {
            content: String::from("CCC"),
            username: String::from("cat"),
            reply: false,
            retweet: true,
        };
        println!("{}", tweet.summarize()); // cat: CCC
    }
}
