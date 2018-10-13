/*
   Common Programming Concepts
       Data Types
           Compound Types
               The Tuple Type
 */

/*
   组合类型（compound type）是将多个类型组合成一个类型，
   rust 有两种组合类型：元组（tuple）和数组（array）；
 */
pub fn fn_03_02_08() {
    println!("-------------------------------------------------- 01");
    {
        /*
           tuple 类型表示多个类型组成的一个类型，多个元素之间使用逗号（,）分隔，
           多个元素使用圆括号（()）包裹；
           tuple 的长度是固定的；
         */
        let tup: (i32, f32, bool) = (9, 3.14, true);

        /*
           可以使用模式匹配（pattern matching）对 tuple 类型的变量进行解构（destructure），
           从而获取该变量的各个成员；
         */
        let (a, b, c) = tup;
        println!("{} {} {}", a, b, c); // 9 3.14 true

        /*
           也可以使用（var.index）的方式，根据索引访问 tuple 中的各个成员；
         */
        println!("{} {} {}", tup.0, tup.1, tup.2); // 9 3.14 true
    }
}
