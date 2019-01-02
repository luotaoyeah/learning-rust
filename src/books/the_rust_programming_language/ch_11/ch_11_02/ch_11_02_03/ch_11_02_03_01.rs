/*
   Writing Automated Tests
       Controlling How Tests Are Run
           Ignoring Some Tests Unless Specifically Requested
 */

pub fn fn_11_02_03_01() {
    println!("-------------------------------------------------- 01");
    {
        /*
           使用 #[ignore] 标记某个测试方法，则默认不会执行该测试方法，
           除非使用参数 -- --ignored
         */
    }
}

#[test]
fn test_01() {}

#[test]
#[ignore]
fn test_02() {}
