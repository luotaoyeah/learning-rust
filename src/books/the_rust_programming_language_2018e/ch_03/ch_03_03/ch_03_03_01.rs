/*
   Functions
 */

pub fn fn_03_03_01() {
    println!("-------------------------------------------------- 01");
    {
        /*
           使用关键字 fn 来声明一个函数，
           函数和变量使用 snake_case 的命名方式（所有字母小写，多个单词之间使用下划线（_）分隔）
         */

        another_function();
    }
}

fn another_function() {
    println!("another_function()");
}
