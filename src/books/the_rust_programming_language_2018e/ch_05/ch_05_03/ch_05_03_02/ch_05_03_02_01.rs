/*
   Struct
       Method Syntax
           Methods with More Parameters
 */

/*
   method 除了第一个参数为 self 以外，还可以定义其他的参数；
 */
pub fn fn_05_03_02_01() {
    println!("-------------------------------------------------- 01");
    {
        struct Rect {
            width: u32,
            height: u32,
        }

        impl Rect {
            /// 是否可以容纳
            fn can_hold(&self, other: &Rect) -> bool {
                self.width > other.width && self.height > other.height
            }
        }

        let rect01 = Rect {
            width: 30,
            height: 40,
        };

        let rect02 = Rect {
            width: 20,
            height: 30,
        };

        let rect03 = Rect {
            width: 20,
            height: 50,
        };

        println!("{}", rect01.can_hold(&rect02)); // true
        println!("{}", rect01.can_hold(&rect03)); // false
    }
}
