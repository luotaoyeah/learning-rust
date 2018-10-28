/*
   Struct
       field init shorthand
 */

pub fn fn_05_01_02_01() {
    println!("-------------------------------------------------- 01");
    {
        /*
           当用来初始化 field 的变量的名称跟 field 的名称相同时，
           可以使用 field init shorthand 语法；
         */

        struct User {
            name: String,
            age: u8,
        }

        let name = String::from("luotao");
        let age = 18;

        // 未使用 field init shorthand
        let user01 = User {
            name: name,
            age: age,
        };

        let user02 = User {
            name: String::from("luotao"),
            // age 使用了 field init shorthand
            age,
        };

        println!("{} {}", user01.name, user02.name); // luotao luotao
    }
}
