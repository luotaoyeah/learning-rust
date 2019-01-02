/*
  More About Cargo and Crates.io
      Publishing a Crate to Crates.io
          Exporting a Convenient Public API with pub use
*/

/*
  通过 pub use 将内部的结构重新输出（reexport）
*/
pub use self::A::S;

pub fn fn_14_01_03_01() {
    println!("-------------------------------------------------- 01");
    {
        /*
          在开发的时候，项目结构可能会分很多模块，可能会有很多嵌套；
          但是在暴露给外界使用时，过多的嵌套和模块会不方便使用，
          因此可以使用 pub use 将内部的结构重新调整，然后输出给外部；
        */
    }
}

pub mod A {
    pub struct S {}
}

pub mod B {
    fn fn01() {
        /*
          可以按重新输出的结构使用 S
        */
        let s01 = super::S {};

        /*
          也可以按原来的结构使用 S
        */
        let s02 = super::A::S {};
    }
}
