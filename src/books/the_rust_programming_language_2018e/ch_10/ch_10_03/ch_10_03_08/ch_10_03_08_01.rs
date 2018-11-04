/*
   Generic Types, Traits，Lifetimes
       Validating References with Lifetimes
           The Static Lifetime
 */

pub fn fn_10_03_08_01() {
    println!("-------------------------------------------------- 01");
    {
        /*
           有一个特殊的 lifetime：'static，表示生命周期为整个程序运行期间；
           所有 string literal 的 lifetime 就是 'static；
         */

        let str01: &'static str = "hello";
    }
}
