/*
   Writing Automated Tests
       How to Write Tests
           The Anatomy of a Test Function
 */

pub fn fn_11_01_01_01() {
    println!("-------------------------------------------------- 01");
    {}
}

/*
   使用 test 标记的函数为测试函数；
 */
#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}

#[test]
fn it_not_works() {
    assert_eq!(2 + 2, 3);
}

#[test]
fn it_panics() {
    panic!("PANIC");
}
