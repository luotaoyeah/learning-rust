/*
   Generic Types, Traits，Lifetimes
       Validating References with Lifetimes
           Lifetime Elision
 */

pub fn fn_10_03_06_01() {
    println!("-------------------------------------------------- 01");
    {
        /*
           函数参数的 lifetime 称之为 input lifetime，
           函数返回值的 lifetime 称之为 output lifetime；


           当函数或者方法没有显式地标注 lifetime parameter 时，
           compiler 会根据三个规则，推测参数和返回值的 lifetime，
           如果三个规则走完之后，所有参数和返回值的 lifetime 都可以确定，则编译通过；
           否则编译出错；

           规则一：
               所有 reference 类型的参数都有自己的 lifetime parameter；
           规则二：
               如果函数只有一个 input lifetime parameter，
               则所有的 output lifetime parameter 就是这个 input lifetime parameter；
           规则三：
               如果函数有多个 input lifetime parameter，且其中一个参数是 &self（或者 &mut self），
               即该函数是一个方法，则所有的 output lifetime parameter 就是 self 的 lifetime parameter；
         */
    }
}

/// first_word() 函数满足规则二，因此可以省略 lifetime parameter 标注；
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
