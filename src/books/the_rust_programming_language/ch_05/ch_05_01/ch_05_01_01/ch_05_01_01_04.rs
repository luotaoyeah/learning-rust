/*
   Struct
       Defining and Instantiating Structs
           structs are expressions
 */

pub fn fn_05_01_04() {
    println!("-------------------------------------------------- 01");
    {
        /*
           struct 的实例是一个 expression；
         */

        struct User {
            name: String,
            age: u8,
            gender: bool,
        }

        fn build_user(name: String, age: u8) -> User {
            User {
                age: age,
                name: name,
                gender: false,
            }
        }

        println!("{}", build_user(String::from("luotao"), 18).name); // luotao
    }
}
