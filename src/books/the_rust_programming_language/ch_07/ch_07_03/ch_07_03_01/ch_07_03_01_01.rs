/*
   Modules
       Referring to Names in Different Modules
 */

pub fn fn_07_03_01_01() {
    println!("-------------------------------------------------- 01");
    {
        /*
           使用 namespace 的方式访问其他 module 中的成员；
         */

        mod a {
            pub mod b {
                pub mod c {
                    pub mod d {
                        pub fn fn_01() {
                            println!("fn_01()");
                        }
                    }
                }
            }
        }

        /*
           如果嵌套很深，则前面的 namespace 会很长；
         */
        a::b::c::d::fn_01(); // fn_01()
    }
}
