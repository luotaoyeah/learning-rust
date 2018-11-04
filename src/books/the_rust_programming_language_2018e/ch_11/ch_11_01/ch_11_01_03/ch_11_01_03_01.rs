/*
   Writing Automated Tests
       How to Write Tests
           Testing Equality with the assert_eq! and assert_ne! Macros
 */

pub fn fn_11_01_03_01() {
    println!("-------------------------------------------------- 01");
    {}
}

fn add_2(i: i32) -> i32 {
    i + 2
}

#[test]
fn it_works() {
    assert_eq!(add_2(2), 4);
}

#[test]
fn it_works_too() {
    assert_ne!(add_2(2), 6);
}

#[test]
fn it_not_works() {
    /*
       assert_eq!() 和 asseret_ne!() 的参数必须实现两个 trait：PartialEq，Debug
     */
    struct A {}

    assert_eq!(A {}, A {}); // [E0369]: binary operation `==` cannot be applied to type
}
