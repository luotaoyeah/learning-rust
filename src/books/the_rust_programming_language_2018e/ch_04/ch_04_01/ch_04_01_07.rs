/*
   Understanding Ownership
       Memory and Allocation
           Stack-Only Data: Copy
 */

pub fn fn_04_01_07() {
    println!("-------------------------------------------------- 01");
    {
        /*
           对于简单类型如 i32，由于数据是完全存储在 stack 上的，
           因此不存在 move 操作，当把 x 赋值给 y 时，发生的是 copy 操作，而不是 move 操作；
         */

        let x = 5;
        let y = x;
        println!("{} {}", x, y); // 5 5
    }

    /*
       rust 定义了一个名叫 Copy 的 trait，凡是使用 Copy 标注了的类型的变量，在赋值给其他变量只有，依然有效；
       如果一个类型使用了 Drop，则它就不能再使用 Copy 了；
       如果一个类型在离开 scope 之后需要做特殊的操作，它也不能使用 Copy；
     */

    /*
       下面这些类型使用了 Copy：
           所有整数，如 u32
           布尔类型，bool
           所有浮点数，如 f32
           字符类型，char
           上面这些类型组成的元祖类型，如 (i32, u32)
     */
}
