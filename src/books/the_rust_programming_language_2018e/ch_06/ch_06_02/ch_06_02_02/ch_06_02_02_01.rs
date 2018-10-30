/*
   Enums and Pattern Matching
       match
           Patterns that Bind to Values
 */

pub fn fn_06_02_02_01() {
    println!("-------------------------------------------------- 01");
    {
        /*
           当作为 match 的 arm 的 enum 具有关联数据时，
           当该 arm 被匹配上时，在后面的语句块中可以访问该 enum 所关联的数据；
         */

        #[derive(Debug)]
        enum UnitedState {
            A,
            B,
            C,
        }

        enum Coin {
            Penny,
            Nickel,
            Dime,
            Quarter(UnitedState),
        }

        fn value_in_cents(coin: Coin) -> u32 {
            match coin {
                Coin::Penny => 1,
                Coin::Nickel => 5,
                Coin::Dime => 10,
                // 当 Coin::Quarter 被匹配时，state 会绑定到它的关联数据
                Coin::Quarter(state) => {
                    println!("quater: {:?}", state);
                    25
                }
            }
        }

        value_in_cents(Coin::Quarter(UnitedState::B)); // quater: B
    }
}
