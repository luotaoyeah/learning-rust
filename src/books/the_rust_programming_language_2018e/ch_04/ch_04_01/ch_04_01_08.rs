/*
   Understanding Ownership
       Ownership and Functions
 */

pub fn fn_04_01_08() {
    println!("-------------------------------------------------- 01");
    {
        /*
           把变量作为参数传递给函数，跟把变量赋值给其他变量一样，
           要么发生 move 操作，要么发生 copy 操作；
         */

        let i: i32 = 5;
        // i32 是简单类型，传递给函数之后，发生 copy 操作
        makes_copy(i);
        println!("{}", i);

        let s: String = String::from("hello");
        // String 类型的变量，传递给函数之后，发生 move 操作
        takes_ownership(s);
        println!("{}", s); // [E0382]: use of moved value: `s`
    }
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}
