/*
   Enums and Pattern Matching
       Enum Values
           Listing 6-2: A Message enum whose variants each store different amounts and types of values
 */

pub fn fn_06_01_02_04() {
    #[derive(Debug)]
    enum Message {
        // 没有关联数据
        Quit,
        // 关联 anonymous struct
        Move { x: i32, y: i32 },
        // 关联 String
        Write(String),
        // 关联 tuple
        ChangeColor(i32, i32, i32),
    }

    println!("-------------------------------------------------- 01");
    {
        println!("{:?}", Message::Quit); // Quit
        println!("{:?}", Message::Move { x: 3, y: 4 }); // Move { x: 3, y: 4 }
        println!("{:?}", Message::Write(String::from("hello"))); // Write("hello")
        println!("{:?}", Message::ChangeColor(0, 0, 0)); // ChangeColor(0, 0, 0)
    }

    println!("-------------------------------------------------- 02");
    {
        /*
            跟 struct 一样，enum 也可以定义 impl block，
            method 中的 self 表示某个 variant 实例；
         */

        impl Message {
            fn log(&self) {
                println!("{:?}", self);
            }
        }

        let msg = Message::Move { x: 3, y: 4 };
        msg.log(); // Move { x: 3, y: 4 }
    }
}
