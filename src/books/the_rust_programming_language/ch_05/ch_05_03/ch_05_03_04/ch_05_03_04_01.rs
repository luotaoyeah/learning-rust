/*
   Struct
       Method Syntax
           Multiple impl Blocks
 */

pub fn fn_05_03_03_01() {
    println!("-------------------------------------------------- 01");
    {
        /*
           一个 struct 可以定义多个 impl block；
         */

        struct Rect {
            width: u32,
            height: u32,
        }

        impl Rect {
            /// 计算面积
            fn calc_area(&self) -> u32 {
                self.width * self.height
            }
        }

        impl Rect {
            /// 创建正方形
            fn square(size: u32) -> Rect {
                Rect {
                    width: size,
                    height: size,
                }
            }
        }
    }
}
