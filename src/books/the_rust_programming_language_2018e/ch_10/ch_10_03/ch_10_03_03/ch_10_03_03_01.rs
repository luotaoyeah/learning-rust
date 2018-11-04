/*
   Generic Types, Traits，Lifetimes
       Validating References with Lifetimes
           Generic Lifetimes in Functions
 */

pub fn fn_10_03_03_01() {
    println!("-------------------------------------------------- 01");
    {
        let str01 = String::from("hello");
        let str02 = "rust";

        let str03 = longest_01(str01.as_str(), str02);
        println!("{}", str03);
    }
}

/// 返回较长的那个 string slice
///
/// [E0106]: missing lifetime specifier
///
/// help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `x` or `y`
fn longest_01(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
