/*
   Understanding Ownership
       Variable Scope
 */

pub fn fn_04_01_02() {
    println!("-------------------------------------------------- 01");
    {
        // 尚未定义 s，及尚未进入 s 的 scope，s 无效；
        println!("{}", s); // [E0425]: cannot find value `s` in this scope

        /*
           对于 string literal 来说，字符串硬编码是保存在栈（stack）上的，
           variable 的 scope 为：从 variable 定义开始，到所在的代码块末尾结束；
         */
        let s = "hello";
    }

    // 离开 s 的 scope，s 无效；
    println!("{}", s); // [E0425]: cannot find value `s` in this scope
}
