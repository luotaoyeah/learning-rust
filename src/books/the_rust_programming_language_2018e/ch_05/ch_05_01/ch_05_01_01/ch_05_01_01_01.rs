/*
   Struct
       Defining and Instantiating Structs
           define a struct
 */

/*
   struct 跟 tuple 有些类似，可以包含多个不同类型的数据；
 */
pub fn fn_05_01_01() {
    println!("-------------------------------------------------- 01");
    {
        /*
            使用关键字 struct 定义一个 struct，
            struct 后面跟上名称，名称一般使用大写字母开头，
            名称后面跟上一对花括号，括号里面包好多个字段（field）的定义，
            每个 field 之间使用逗号分隔；
         */

        struct User {
            username: String,
            email: String,
            age: u8,
            active: bool,
        }
    }
}
