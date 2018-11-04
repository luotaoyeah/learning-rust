/*
   Writing Automated Tests
       How to Write Tests
           Using Result<T, E> in tests
 */

pub fn fn_11_01_06_01() {
    println!("-------------------------------------------------- 01");
    {}
}

/*
   默认情况，测试方法通过是否 panic 来判断是否通过，
   测试方法可以返回 Result<T, E>，通过返回为 Ok 还是 Err 来判断是否通过；
 */
#[test]
fn test_01() -> Result<(), String> {
    if 2 + 2 == 4 {
        Ok(())
    } else {
        Err(String::from("2 + 2 != 4"))
    }
}
