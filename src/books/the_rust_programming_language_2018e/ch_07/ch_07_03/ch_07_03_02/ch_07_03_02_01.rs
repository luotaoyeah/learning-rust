/*
   Modules
       Referring to Names in Different Modules
           Bring Names into Scope with the `use` Keyword
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

pub fn fn_07_03_02_01() {
    println!("-------------------------------------------------- 01");
    {
        a::b::c::d::fn_01(); // fn_01()

        /*
           使用 use 关键字，将某个 namespace 引入当前 scope，
           然后就可以直接使用这个 namespace；
         */
        use self::a::b::c::d;
        d::fn_01(); // fn_01()
    }
}
