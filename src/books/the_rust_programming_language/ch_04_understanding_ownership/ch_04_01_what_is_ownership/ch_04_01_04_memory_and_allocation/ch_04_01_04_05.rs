//
// Understanding Ownership
//     Memory and Allocation
//         Stack-Only Data: Copy
//

pub fn fn_04_01_04_05() {
    println!("-------------------------------------------------- 01");
    {
        // integer 是简单类型，数据是存放在 stack 上的，
        // 当把一个 integer 变量赋值给另外一个 integer 变量时，会在 stack 上复制一份数据
        // 这里发生的操作称之为 copy

        let i01: i32 = 9;
        let i02: i32 = i01;

        println!("{}", i01); // 9
        println!("{}", i02); // 9
    }

    println!("-------------------------------------------------- 02");
    {
        // 之所以会发生 copy 操作，而不是 move 操作，
        // 是因为在该类型上标注了一个特殊的 trait：Copy，
        // 我们也可以在自定义的类型上标注 Copy 特性，从而实现 copy 操作

        // 如下，类型 A 没有标注 Copy 特性，因此在赋值时发生的是 move 操作，
        #[derive(Debug)]
        struct A {
            x: i32,
        }

        let a01: A = A { x: 9 };
        let a02: A = a01;
        println!("{:?}", a02);
        /*
                println!("{:?}", a01); // [E0382]: borrow of moved value: `a01`
        */
    }

    println!("-------------------------------------------------- 03");
    {
        // 类型 B 标注了 Copy 特性，因此在赋值时发生的是 copy 操作
        #[derive(Debug, Copy, Clone)]
        struct B {
            x: i32,
        }

        impl PartialEq for B {
            fn eq(&self, other: &Self) -> bool {
                self.x == other.x
            }
        }

        let b01: B = B { x: 9 };
        let b02: B = b01;
        println!("{:?}", b02); // B { x: 9 }
        println!("{:?}", b01); // B { x: 9 }

        println!("{}", b01 == b02); // true
    }
}
