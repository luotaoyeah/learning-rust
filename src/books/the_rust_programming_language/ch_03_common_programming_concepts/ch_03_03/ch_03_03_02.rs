/*
   Functions
       Function Parameters
 */

pub fn fn_03_03_02() {
    println!("-------------------------------------------------- 01");
    {
        // 函数的参数在函数调用中称之为'实参（argument）'
        another_function(5);
        fn_03(6, false);
    }
}

/*
   函数的参数在函数定义中称之为'形参（parameter）'，
   函数的参数必须指定类型；
 */
fn another_function(x: i32) {
    println!("x: {}", x);
}

fn fn_03(x: i32, y: bool) {
    println!("x: {}", x);
    println!("y: {}", y);
}
