/*
  Doing Arithmetic
      Floating-Poing Arithmetic
          must be the same type
*/

pub fn fn_02_03_02() {
    println!("-------------------------------------------------- 01");
    {
        // 在 rust 中，integer 和 float 不能直接进行计算，否则编译报错
        /*
                println!("{}", 3.14 * 2); // [E0277]: cannot multiply `{integer}` to `{float}`
        */

        // f32 和 f64 也不能直接进行计算，否则编译报错
        /*
                println!("{}", 3.14f64 * 2.0f32); // expected f64, found f32
        */

        // 必须显式地将 integer 转换为 float 之后，才能进行计算
        println!("{}", 3.14 * (2 as f64)); // 6.28
    }
}
