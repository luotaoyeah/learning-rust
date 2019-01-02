/*
   Writing Automated Tests
       How to Write Tests
           Checking for Panics with should_panic
 */

pub fn fn_11_01_05_02() {
    println!("-------------------------------------------------- 01");
    {}
}

/*
   默认情况，should_panic 会通过所有的 panic，
   如果希望通过某些特定的 panic，可以声明 expected 参数，
   当 failure message 包含 expected 参数时，测试通过；
 */

#[test]
#[should_panic]
fn test_01() {
    panic!("A")
}

#[test]
#[should_panic(expected = "A")]
fn test_02() {
    panic!("AAA")
}

#[test]
#[should_panic(expected = "B")]
fn test_03() {
    panic!("A")
}
