/*
   Generic Types, Traits，Lifetimes
       Generic Data Types
           In Method Definitions
 */

pub fn fn_10_01_04_01() {
    println!("-------------------------------------------------- 01");
    {
        struct Point<T> {
            x: T,
            y: T,
        }

        /*
           在 impl 后面声明泛型参数：impl<T>，
           这样，compiler 会将 Point<T> 中的 T 识别为 generic type，而不是 concrete type；
         */
        impl<T> Point<T> {
            fn x(&self) -> &T {
                &self.x
            }
        }

        let point: Point<i32> = Point { x: 9, y: 9 };
        println!("{}", point.x()); // 9
    }
}
