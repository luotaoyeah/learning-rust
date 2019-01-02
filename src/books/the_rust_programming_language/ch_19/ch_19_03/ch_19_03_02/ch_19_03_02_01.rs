/*
  Advanced Features
      Advanced Traits
          Default Generic Type Parameters and Operator Overloading
*/

use std::ops::Add;

pub fn fn_19_03_02_01() {
    println!("-------------------------------------------------- 01");
    {
        /*
           generic type 可以提供一个默认类型；
        */

        struct Foo<T = i32> {
            bar: T,
        }

        let f = Foo { bar: 9 };
        println!("{}", f.bar); // 9
    }

    println!("-------------------------------------------------- 02");
    {
        /*
          通过实现 std::ops 中的 trait，可以重载某些运算符（operator overloading）；
        */

        #[derive(Debug)]
        struct Point {
            x: i32,
            y: i32,
        }

        impl Add for Point {
            type Output = Point;

            fn add(self, rhs: Point) -> Point {
                Point {
                    x: self.x + rhs.x,
                    y: self.y + rhs.y,
                }
            }
        }

        let p01 = Point { x: 1, y: 1 };
        let p02 = Point { x: 2, y: 2 };

        let p03 = p01 + p02;
        println!("{:?}", p03); // Point { x: 3, y: 3 }
    }
}
