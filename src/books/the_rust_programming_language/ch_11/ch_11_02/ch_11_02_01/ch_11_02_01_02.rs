/*
   Writing Automated Tests
       Controlling How Tests Are Run
           Showing Function Output
 */

pub fn fn_11_02_01_02() {
    println!("-------------------------------------------------- 01");
    {
        /*
           默认情况，如果一个测试方法通过了，
           则它输出到标准输出的信息会被 capture 住，不会打印，
           可以通过 -- --nocapture 参数取消此行为；
         */
    }
}

#[test]
fn test_01() {
    println!("HELLO");
    assert_eq!(2 + 2, 4);
}
