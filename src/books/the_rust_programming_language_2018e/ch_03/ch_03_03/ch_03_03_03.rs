/*
   Functions
       Function Bodies
       Statements and Expressions
 */

pub fn fn_03_03_03() {
    println!("-------------------------------------------------- 01");
    {
        /*
           函数体中包含一系列的语句（statement），以及一个可选的表达式（expression）结尾；
         */

        /*
           expression 会计算出一个值，
           statement 是一系列指令，不会计算出一个值；
         */

        /*
           赋值语句是一种简单的 statement；
         */
        let y: i32 = 6;

        /*
           函数定义也是一种 statement；
         */
        fn fn_01() {}
    }

    println!("-------------------------------------------------- 02");
    {
        /*
           因为 statement 不会计算出一个值，因此不能将 statement 赋值给一个变量；
         */

        /*
                let x = (let y = 6); // error: expected expression, found statement (`let`)
                let x = y = 6; // error[E0425]: cannot find value `y` in this scope
        */
    }

    println!("-------------------------------------------------- 03");
    {
        /*
           statement 通常由 expression 组成；
         */

        // 赋值语句中 5 + 6 是一个 expression
        let y = 5 + 6;

        /*
           函数调用是一个 expression；
         */
        fn fn_01() {}
        let x = fn_01();

        /*
           宏（macro）调用是一个 expression；
         */
        let z = println!("y: {}", y);
    }

    println!("-------------------------------------------------- 04");
    {
        /*
           代码块（block）是一个 expression；
         */

        let y_01 = {
            let x = 3;
            /*
               此处 x + 1 是一个 expression，后面没有加分号，它的值会返回作为整个 block 的值；
             */
            x + 1
        };
        println!("y_01: {}", y_01); // y_01: 4

        let y_02 = {
            let x = 3;
            /*
               如果加上分号，则 x + 1; 就变成了一个 statement，不会返回；
             */
            x + 1;
        };
        /*
                println!("y_02: {}", y_02); // error[E0277]: `()` doesn't implement `std::fmt::Display`
        */
    }
}
