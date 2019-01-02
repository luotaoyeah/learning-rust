/*
   Generic Types, Traits，Lifetimes
       Generic Data Types
           In Method Definitions
 */

pub fn fn_10_01_04_03() {
    println!("-------------------------------------------------- 01");
    {
        /*
           generic impl block 中可以定义其他的 generic method，
           他们的 generic type 可以不同；
         */

        #[derive(Debug)]
        struct Point<T, U> {
            x: T,
            y: U,
        }

        impl<T, U> Point<T, U> {
            fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
                Point {
                    x: self.x,
                    y: other.y,
                }
            }
        }

        let point01: Point<i32, f32> = Point { x: 3, y: 4.0 };
        let point02: Point<&str, char> = Point { x: "A", y: 'B' };

        println!("{:?}", point01.mixup(point02)); // Point { x: 3, y: 'B' }
    }
}
