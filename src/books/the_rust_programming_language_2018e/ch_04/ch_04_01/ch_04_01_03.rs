/*
   Understanding Ownership
       The String Type
 */

/*
   对于简单类型，如：整数（i32，u32），浮点数（f32，f64），bool，char 等，都是保存在 stack 上的，
   当离开 scope 时，会从 stack 中 pop 出来；

   对于复杂类型，如：String，都是保存在 heap 上的；
 */

/*
   string literal 是 hardcode 的，是 immutable 的，是长度固定的；
 */

pub fn fn_04_01_03() {
    println!("-------------------------------------------------- 01");
    {
        // 使用 String::from() 方法，从一个 string literal 创建一个 String 变量
        let mut s: String = String::from("hello");
        // 使用 push_str() 方法，往 s 中添加字符串；
        s.push_str(" world");
        println!("{}", s); // hello world
    }
}
