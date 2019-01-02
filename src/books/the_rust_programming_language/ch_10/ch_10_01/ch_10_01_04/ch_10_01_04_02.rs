/*
   Generic Types, Traits，Lifetimes
       Generic Data Types
           In Method Definitions
 */

pub fn fn_10_01_04_02() {
    println!("-------------------------------------------------- 01");
    {
        /*
           impl 后面没有声明泛型参数，表示是对某个具体类型的实现；
         */

        struct Point<T> {
            x: T,
            y: T,
        }

        impl Point<f32> {
            fn distance(&self) -> f32 {
                (self.x.powi(2) + self.y.powi(2)).sqrt()
            }
        }

        let point: Point<f32> = Point { x: 3.0, y: 4.0 };
        println!("{}", point.distance()); // 5
    }
}
