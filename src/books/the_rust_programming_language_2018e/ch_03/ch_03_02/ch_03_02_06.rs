/*
   Common Programming Concepts
       Data Types
           Scalar Types
               The Boolean Type
 */

/*
   布尔类型使用 bool 标识，可选值为 true 和 false，占用一个字节（byte）；
 */
pub fn fn_03_02_06() {
    println!("-------------------------------------------------- 01");
    {
        let a = true;
        let b: bool = false;

        println!("{}, {}", a, b); // true, false
    }

    println!("-------------------------------------------------- 02");
    {
        /*
           布尔值主要用在条件语句中
         */
        let gt: bool = 3 > 2;
        if gt {
            println!(">");
        } else {
            println!("<")
        }
    }
}
