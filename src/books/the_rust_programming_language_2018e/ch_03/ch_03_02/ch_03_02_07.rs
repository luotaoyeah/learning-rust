/*
   Common Programming Concepts
       Data Types
           Scalar Types
               The Character Type
 */

pub fn fn_03_02_07() {
    println!("-------------------------------------------------- 01");
    {
        /*
           char 用来标识字符类型，字符类型的值为单个 unicode 字符，使用单引号包裹；
         */

        let a: char = 'A';
        let b: char = '♡';
        let c: char = '😶';

        println!("{} {} {}", a, b, c);
    }
}
