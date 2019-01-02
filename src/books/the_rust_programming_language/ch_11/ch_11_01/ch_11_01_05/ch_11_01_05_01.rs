/*
   Writing Automated Tests
       How to Write Tests
           Checking for Panics with should_panic
 */

pub fn fn_11_01_05_01() {
    println!("-------------------------------------------------- 01");
    {}
}

/*
   should_panic 用来标记：期望某个测试方法会 panic，
   即如果不 panic，则测试不通过；
 */
#[test]
#[should_panic]
fn it_panics() {
    panic!("");
}

#[test]
#[should_panic]
fn it_not_panics() {}
