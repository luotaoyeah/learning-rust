/*
   Struct
       Create Instances From Other Instances with Struct Update Syntax
 */

pub fn fn_05_01_03_01() {
    /*
       有这样的场景：
           创建一个新的实例，它的某些属性值来自另外一个实例；
     */

    struct User {
        name: String,
        age: u8,
        email: String,
    }

    println!("-------------------------------------------------- 01");
    {
        let user01: User = User {
            name: String::from("luotao"),
            age: 18,
            email: String::from("xxx@gmail.com"),
        };

        let user02 = User {
            name: String::from("tom"),
            age: user01.age,
            email: user01.email,
        };

        println!("{}", user02.email); // xxx@gmail.com
    }

    println!("-------------------------------------------------- 02");
    {
        /*
           使用 struct update 语法，可以简化代码；
         */

        let user01: User = User {
            name: String::from("luotao"),
            age: 18,
            email: String::from("xxx@gmail.com"),
        };

        // ..user01 表示 user02 中没有显式指定的 field 都从 user01 中来；
        let user02 = User {
            name: String::from("tom"),
            ..user01
        };

        println!("{}", user02.email); // xxx@gmail.com
    }
}
