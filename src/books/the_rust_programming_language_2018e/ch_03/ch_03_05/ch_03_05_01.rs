/*
   Control Flow
       if expressions
 */

pub fn fn_03_05_01() {
    println!("-------------------------------------------------- 01");
    {
        /*
           使用 if/else 实现条件语句；
           if 后面的条件不需要使用圆括号包裹起来；
           if 代码块和 else 代码块称为 arm（类似 match 语句的 arm）；
         */

        fn fn_01(n: i32) {
            if n < 5 {
                println!("true");
            } else {
                println!("false");
            }
        }

        fn_01(3);
        fn_01(7);
    }

    println!("-------------------------------------------------- 02");
    {
        /*
           if 后面的条件表达式必须是 bool 类型；
         */

        let b: bool = 3 < 4;
        let i: i32 = 9;

        if b {
            println!("good");
        }

        /*
           变量 i 是 i32 类型，不会自动转换为 bool 类型；
         */
        // expected bool, found i32
        if i {
            println!("bad");
        }
    }
}
