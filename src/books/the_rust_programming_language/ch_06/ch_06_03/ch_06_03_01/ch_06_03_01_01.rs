/*
   Enums and Pattern Matching
       Concise Control Flow with `if let`
 */

pub fn fn_06_03_01_01() {
    println!("-------------------------------------------------- 01");
    {
        /*
           因为 match 是 exhaustive 的，必须列出所有可能的情况，
           当只需要关注某些情况时，可以使用 _ placeholder 匹配其他的情况，
           当只需要关注某个情况时，还可以使用 `if let` 语法糖；
         */

        match_value_01(Some(3u8)); // three
        match_value_01(Some(4u8)); // not three
        match_value_02(Some(3u8)); // three
        match_value_02(Some(4u8)); // not three
    }
}

/// 使用 match 和 _
fn match_value_01(x: Option<u8>) {
    match x {
        Some(3) => println!("three"),
        _ => println!("not three"),
    }
}

// 使用 `if let`
fn match_value_02(x: Option<u8>) {
    if let Some(3) = x {
        println!("three");
    } else {
        println!("not three");
    }
}
