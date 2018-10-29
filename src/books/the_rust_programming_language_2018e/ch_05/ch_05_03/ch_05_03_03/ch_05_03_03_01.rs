/*
   Struct
       Method Syntax
           Associated Functions
 */

pub fn fn_05_03_03_01() {
    println!("-------------------------------------------------- 01");
    {
        /*
           在 impl block 中定义的函数，如果第一个参数不是 self，
           则它是一个 associated function，类似其他语言的静态方法；
           associated function 不是 method，因为它没有关联的实例对象；
         */

        #[derive(Debug)]
        struct Rect {
            width: u32,
            height: u32,
        }

        impl Rect {
            fn square(size: u32) -> Rect {
                Rect {
                    width: size,
                    height: size,
                }
            }
        }

        let rect = Rect::square(10);

        println!("{:#?}", rect);
    }
}
