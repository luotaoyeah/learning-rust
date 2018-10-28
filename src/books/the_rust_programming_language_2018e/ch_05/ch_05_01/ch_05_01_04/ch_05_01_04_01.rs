/*
   Struct
       tuple struct
 */

pub fn fn_05_01_04_01() {
    println!("-------------------------------------------------- 01");
    {
        /*
           tuple struct 是一种特殊的 struct，将 tuple 和 struct 结合，
           tuple struct 的 field 没有名字；
         */

        struct Color(i32, i32, i32);
        struct Point(i32, i32, i32);

        let color = Color(255, 255, 255);
        let point = Point(1, 2, 3);

        /*
           tuple struct 的用法跟普通的 tuple 一样，可以通过 tuple.index 的方式访问成员，
         */
        println!("{}", color.0); // 255

        /*
            可以使用 destructure 的方式解构成员；
         */
        let Point(x, y, z) = point;
        println!("{}", y); // 2
    }
}
