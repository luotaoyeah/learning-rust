/*
  Advanced Features
      Advanced Lifetimes
          Lifetime Bounds on References to Generic Types
*/

pub fn fn_19_02_01_02() {
    println!("-------------------------------------------------- 01");
    {
        /*
          除了可以使用 trait bounds 对 generic type 进行限制，
          还可以使用 lifetime bounds 对 generic type 进行限制；
        */

        struct Ref<'a, T>(&'a T)
        where
            T: 'a;
    }
}
