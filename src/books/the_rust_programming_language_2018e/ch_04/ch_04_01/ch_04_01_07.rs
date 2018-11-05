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

    println!("-------------------------------------------------- 02");
    {
        /*
           rust 定义了一个名叫 Copy 的 trait，
               凡是实现了 Copy 的类型，在赋值给其他变量时，会发生 copy 操作，而不是 move 操作；
           如果一个类型实现了 Drop，则它就不能再实现 Copy 了；
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

        /*
             #[derive(Debug)]
             struct A {
                 a: i32,
             };

             let a01: A = A { a: 0 };
             let a02: A = a01;
             println!("{:?}", a01); // [E0382]: use of moved value: `a01`
         */

        #[derive(Debug, Clone, Copy)]
        struct B {
            b: i32,
        };

        let b01: B = B { b: 0 };
        let b02: B = b01;
        /*
           B 实现了 Copy，在赋值操作时发生的时 copy 操作，而不是 move 操作；
         */
        println!("{:?}", b01); // B { b: 0 }
    }
}
