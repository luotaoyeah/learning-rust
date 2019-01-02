/*
   Struct
       An Example Program Using Struct
 */

/*
   计算矩形的面积；
 */
pub fn fn_05_02_01_01() {
    println!("-------------------------------------------------- 01");
    {
        let area = calc_area(3, 4);
        println!("{}", area); // 12
    }
}

/// 计算矩形面积
/// * `width` - 宽度
/// * `height` - 高度
fn calc_area(width: u32, height: u32) -> u32 {
    width * height
}
