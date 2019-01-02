/*
   Writing Automated Tests
       Controlling How Tests Are Run
           Running a Subset of Tests by Name
               Running Single Tests
               Filtering to Run Multiple Tests
 */

pub fn fn_11_02_02_01() {
    println!("-------------------------------------------------- 01");
    {
        /*
            指定测试方法名称，则只会执行该测试方法：`cargo test test_01`；
            指定名称的一部分，则包含该部分名称的测试方法会执行：`cargo test e`
                此处会使用测试方法的完整路径进行匹配，即包含模块路径（如：a::e::c::test_01()）；
         */
    }
}

#[test]
fn test_01() {}

#[test]
fn test_02() {}

#[test]
fn test_03() {}
