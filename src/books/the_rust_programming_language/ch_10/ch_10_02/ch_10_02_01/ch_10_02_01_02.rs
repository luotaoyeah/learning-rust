/*
   Generic Types, Traits，Lifetimes
       Traits
           Defining a Trait
           Implementing a Trait on a Type
 */

pub fn fn_10_02_01_02() {
    println!("-------------------------------------------------- 01");
    {
        /*
           trait 有一个限制：trait 和实现 trait 的 type，必须有一个是在当前 crate 中定义的；
         */

        pub trait MyTrait {
            fn print_len(&self) -> String;
        }

        impl<T> MyTrait for Vec<T> {
            fn print_len(&self) -> String {
                let mut len = String::from("len: ");

                len.push_str(&(self.len().to_string()));

                len
            }
        }

        let vec = vec![1, 2, 3];
        println!("{}", vec.print_len()); // len: 3
    }
}
