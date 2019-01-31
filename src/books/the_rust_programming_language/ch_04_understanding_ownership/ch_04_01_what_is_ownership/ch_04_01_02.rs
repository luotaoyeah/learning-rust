//
// Understanding Ownership
//     Variable Scope
//

pub fn fn_04_01_02() {
    println!("-------------------------------------------------- 01");
    // 变量的作用域，指的是程序中的某个范围，
    // 在这个范围中，这个变量是有效的，离开这个范围，变量是无效的
    {
        // 在变量定义之前，变量是无效的，此处不是变量的作用域
        /*
                println!("{}", s01); // [E0425]: cannot find value `s01` in this scope
        */

        let s01: &str = "rust";
        println!("{}", s01);
    }

    // 离开了 s01 所在的代码块，即离开了 s01 的作用域，变量 s01 就无效了
    /*
        println!("{}", s01); // [E0425]: cannot find value `s01` in this scope
    */
}
