/*
   Writing Automated Tests
       Test Organization
           Integration Tests for Binary Crates
 */

pub fn fn_11_03_02_02() {
    println!("-------------------------------------------------- 01");
    {
        /*
           对于 binary crate（即只包含 src/main.rs）的项目，不能进行集成测试，
           只能对 library crate 进行集成测试，
           所以对于 binary crate，通常会将主要的代码放到 src/lib.rs 中，然后在 src/main.rs 中进行调用，
           这样就可以方便的对 src/lib.rs 进行集成测试；
         */
    }
}
