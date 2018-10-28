/*
   Struct
       An Example Program Using Struct
           Adding Useful Functionality with Derived Traits
*/

pub fn fn_05_02_04_01() {
    println!("-------------------------------------------------- 01");
    {
        ///
        struct Rect {
            width: u32,
            height: u32,
        }

        let rect = Rect {
            width: 3,
            height: 4,
        };

        /*
           在 println!() 中，占位符 {} 表示使用 std::fmt::Display 来格式化；
           因为 Rect 没有实现 std::fmt::Display，
           因此不能使用占位符 {} 的方式来打印信息；
         */
        /*
                println!("rect: {}", rect); // [E0277]: `Rect` doesn't implement `std::fmt::Display`
        */

        /*
           在 println!() 中，占位符 {:?} 表示使用 std::fmt::Debug 来格式化；
           因为 Rect 没有实现 std::fmt::Debug，
           所以不能使用占位符 {:?} 的方式来打印信息；
         */
        /*
                println!("rect: {:?}", rect); // [E0277]: `Rect` doesn't implement `std::fmt::Debug`
        */
    }

    println!("-------------------------------------------------- 02");
    {
        /*
           使用 #[derive(Debug)] 标注，表示实现 std::fmt::Debug 这个 trait；
         */
        #[derive(Debug)]
        struct Rect02 {
            width: u32,
            height: u32,
        }

        let rect = Rect02 {
            width: 3,
            height: 4,
        };

        println!("rect: {:?}", rect); // rect: Rect02 { width: 3, height: 4 }

        /*
           rect: Rect02 {
               width: 3,
               height: 4
           }
         */
        println!("rect: {:#?}", rect);

        println!("{:?}", [1, 2, 3]); // [1, 2, 3]
    }
}
