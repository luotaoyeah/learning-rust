/*
   Enums and Pattern Matching
       match
           Listing 6-3: An enum and a match expression that has the variants of the enum as its patterns
 */

pub fn fn_06_02_01_01() {
    println!("-------------------------------------------------- 01");
    {
        /*
           rust 中的 match 流程控制语句，类似于其他语言的 switch 语句，
           里面的每一个分支称之为一个 arm，类似于 switch 语句的 case 分支，
           一个 arm 包含两个部分：pattern 和 expression，使用 => 分隔；
         */

        enum Coin {
            Penny,
            Nickel,
            Dime,
            Quater,
        }

        fn value_in_cents(coin: Coin) -> u32 {
            match coin {
                Coin::Penny => 1,
                Coin::Nickel => 5,
                // => 后面包含多条语句时，可以使用 {} 包裹起来；
                Coin::Dime => {
                    println!("DIME");
                    10
                }
                Coin::Quater => 25,
            }
        }

        println!("{}", value_in_cents(Coin::Dime)); // 10
    }
}
