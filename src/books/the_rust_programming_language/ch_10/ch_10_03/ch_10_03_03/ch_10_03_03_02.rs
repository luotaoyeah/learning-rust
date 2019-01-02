/*
   Generic Types, Traits，Lifetimes
       Validating References with Lifetimes
           Lifetime Annotation Syntax
           Lifetime Annotations in Function Signatures
 */

pub fn fn_10_03_03_01() {
    println!("-------------------------------------------------- 01");
    {
        /*
           lifetime annotation 用于描述多个 reference 的 lifetime 之间的关系；
           generic lifetime parameter 属于一种约束，
           它告诉 borrow cheker 在编译时期需要进行某些检查；
           它并不会改变所描述的 reference 的 lifetime；
         */

        let str01 = String::from("hello");
        let str02 = "rust";

        println!("{}", longest(str01.as_str(), str02)); // hello
    }
}

/*
   generic lifetime parameter 的声明方式和 generic type parameter 类似；
   下面的 generic lifetime parameter（'a）表示：x 和 y 以及返回值的 lifetime 是一样的；
   在实际调用 longest() 函数的时候，'a 的值为 x 和 y 的 lifetime 中较短的那一个；
 */
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
