/*
   Struct
       Method Syntax
           Defining Methods
 */

/*
   method 跟 function 类似，但是是定义在 struct，enum，trait 上下文中的，
   且第一个参数必须是 self ，表示调用该 method 的实例；
 */

#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32,
}

/*
   使用关键字 impl 定义一个 impl block，表示里面的内容是定义在 Rect 上下文中的；
 */
impl Rect {
    fn calc_area(&self) -> u32 {
        self.width * self.height
    }
}

pub fn fn_05_02_01_01() {
    println!("-------------------------------------------------- 01");
    {
        let rect = Rect {
            width: 3,
            height: 4,
        };

        /*
           使用 obj.method() 的形式调用方法；
         */
        println!("{}", rect.calc_area()); // 12
    }
}
