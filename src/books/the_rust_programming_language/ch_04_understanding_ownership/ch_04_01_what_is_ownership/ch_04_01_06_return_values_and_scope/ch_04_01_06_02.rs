//
// Understanding Ownership
//     Return Values and Scope
//

pub fn fn_04_01_06_02() {
    println!("-------------------------------------------------- 01");
    {
        // 可以将变量作为参数传入函数之后，再重新返回该变量，即执行两次 move 操作，
        // 从而实现对变量的再次访问

        let mut s01: String = String::from("FOO");
        let (s01, len) = fn_01(s01);

        println!("{}", s01); // FOO BAR
        println!("{}", len); // 7
    }
}

///
fn fn_01(mut s: String) -> (String, usize) {
    s.push_str(" BAR");
    let len: usize = s.len();
    (s, len)
}
