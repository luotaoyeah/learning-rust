/*
   Struct
       Defining and Instantiating Structs
           access struct's fields
 */

pub fn fn_05_01_03() {
    println!("-------------------------------------------------- 01");
    {
        /*
           可以通过 struct.field 的形式，获取某个 field 的值，或者修改某个 field 的值；
         */

        struct User {
            name: String,
            age: u8,
        }

        /*
           要修改 struct 的某个 field 的值，必须将 struct 设置为 mut，
           不能单独设置某个 field 为 mut；
         */
        let mut user01: User = User {
            name: String::from("luotao"),
            age: 18,
        };

        println!("{}", user01.name); // luotao

        user01.age = 20;
        println!("{}", user01.age); // 20
    }
}
