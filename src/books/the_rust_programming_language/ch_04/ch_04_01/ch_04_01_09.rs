/*
   Understanding Ownership
       Ownership and Functions
           Return Values and Scope
 */

pub fn fn_04_01_09() {
    println!("-------------------------------------------------- 01");
    {
        /*
           函数返回的时候，也会发生 ownership 的转移；
         */

        let s01 = gives_ownership();

        let s02 = String::from("hello");
        let s03 = takes_and_gives_back(s02);
    }

    println!("-------------------------------------------------- 02");
    {
        /*
           当把变量作为参数传递给函数时，会发生 move 操作，变量会变得无效，
           如果希望把变量传递给函数之后依然有效，需要从函数中将变量再返回出来；
         */

        /// 获取长度
        fn get_length(s: String) -> (String, usize) {
            let length = s.len();
            // 通过返回 tuple 的方式返回多个值
            (s, length)
        }

        let s01 = String::from("hello");
        // 对 tuple 进行 destructure
        let (s02, len) = get_length(s01);

        println!("{} {}", s02, len); // hello 5
    }
}

fn gives_ownership() -> String {
    let some_string: String = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
