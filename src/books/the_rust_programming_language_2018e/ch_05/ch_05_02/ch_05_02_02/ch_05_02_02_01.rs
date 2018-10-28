/*
   Struct
       An Example Program Using Struct
           Refactoring with Tuples
*/

/*
   计算矩形的面积；
 */
pub fn fn_05_02_02_01() {
    println!("-------------------------------------------------- 01");
    {
        let area = calc_area((3, 4));
        println!("{}", area); // 12
    }
}

/// 计算面积
/// * `rect` 矩形
fn calc_area(rect: (u32, u32)) -> u32 {
    rect.0 * rect.1
}
