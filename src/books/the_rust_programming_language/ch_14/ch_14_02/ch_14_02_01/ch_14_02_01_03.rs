/*
  More About Cargo and Crates.io
      Publishing a Crate to Crates.io
          Commenting Contained Items
*/

pub fn fn_14_01_02_03() {
    println!("-------------------------------------------------- 01");
    {
        /*
          以 //！ 开头的注释通常用来描述它所在的 crate 或者 mod；
        */

        mod mod_01 {
            //!
            //! this is a mod comments.
            //!

            ///
            /// this is a documentation comment.
            ///
            pub fn fn_01() {}
        }
    }
}
