/*
   Generic Types, Traits，Lifetimes
       Generic Data Types
           In Struct Definitions
 */

pub fn fn_10_01_02_01() {
    println!("-------------------------------------------------- 01");
    {
        #[derive(Debug)]
        struct Point<T, U> {
            x: T,
            y: U,
        }

        let point01: Point<i32, i32> = Point { x: 1, y: 1 };
        println!("{:?}", point01); // Point { x: 1, y: 1 }

        let point02: Point<f32, f32> = Point { x: 1.0, y: 1.0 };
        println!("{:?}", point02); // Point { x: 1.0, y: 1.0 }

        let point03: Point<char, &str> = Point { x: 'A', y: "A" };
        println!("{:?}", point03); // Point { x: 'A', y: "A" }
    }
}
