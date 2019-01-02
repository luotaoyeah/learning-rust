/*
   Struct
       An Example Program Using Struct
           Refactoring with Struct
*/

/*
   计算矩形的面积；
 */
pub fn fn_05_02_03_01() {
    println!("-------------------------------------------------- 01");
    {
        let rect = Rect {
            width: 3,
            height: 4,
        };

        let area = calc_area(&rect);
        println!("{}", area); // 12
    }
}

struct Rect {
    width: u32,
    height: u32,
}

/// 计算面积
/// * `rect` 矩形
fn calc_area(rect: &Rect) -> u32 {
    rect.width * rect.height
}
