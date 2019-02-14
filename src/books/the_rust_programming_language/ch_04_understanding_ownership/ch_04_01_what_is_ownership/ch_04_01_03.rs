//
// Understanding Ownership
//     The `String` Type
//

//
// 对于简单的类型，如：i32，f64，bool，char 等，都是保存在 stack 上的，
// 当离开它们的 scope 时，会从 stack 中 pop 出来
//
// 对于复杂的类型，如：String，都是保存在 heap 上的
//

//
// string literal 是硬编码在程序代码中的，是 immutable 的，是长度固定的
//

pub fn fn_04_01_03() {
    let str01: String = fn_01();
    fn_02(str01);
}

///
fn fn_01() -> String {
    println!("-------------------------------------------------- 01");
    // 可以使用 String::from() 方法，从一个 string literal 中创建一个 String
    let s01: &str = "rust";
    println!("{}", s01); // rust
    crate::print_type(&s01); // &str

    let mut str01: String = String::from(s01);
    println!("{}", str01); // rust
    crate::print_type(&str01); // std::string::String

    str01
}

///
fn fn_02(mut str01: String) {
    println!("-------------------------------------------------- 02");
    // String 类型的变量是保存在 heap 上的，大小是不固定的，可以对其进行扩充
    str01.push_str(" cargo");

    str01.push(' ');
    str01.push('w');
    str01.push('a');
    str01.push('s');
    str01.push('m');

    println!("{}", str01); // rust cargo wasm
}
