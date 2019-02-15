//
// Understanding Ownership
//     Return Values and Scope
//

pub fn fn_04_01_06_01() {
    println!("-------------------------------------------------- 01");
    {
        // 从函数中返回数据时，也会发生 ownership 的转移

        // 如下，在 fn_01() 中创建的变量 s01 被返回之后，被 move 到了 s02 中
        let s02: String = fn_01();
        println!("{}", s02); // FOO BAR
    }
}

///
fn fn_01() -> String {
    let mut s01: String = String::from("FOO");
    s01.push_str(" BAR");
    s01
}
