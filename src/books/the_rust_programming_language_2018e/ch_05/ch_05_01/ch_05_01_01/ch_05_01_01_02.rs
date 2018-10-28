/*
   Struct
       Defining and Instantiating Structs
           instantiate a struct
 */

pub fn fn_05_01_02() {
    println!("-------------------------------------------------- 01");
    {
        struct User {
            username: String,
            email: String,
            age: u8,
            active: bool,
        }

        /*
            struct 在定义好之后，可以创建实例，
            在实例化的时候，fields 的顺序可以跟定义时候的顺序不一致；
         */

        let user01 = User {
            active: true,
            age: 18,
            email: String::from("xxx@gmail.com"),
            username: String::from("luotao"),
        };
    }
}
