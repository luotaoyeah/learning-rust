/*
   Understanding Ownership
       The Slice Type
           Other Slices
 */

pub fn fn_04_03_05_01() {
    println!("-------------------------------------------------- 01");
    {
        /*
           除了 string slice 之外，对于其他类型的数组，也可以使用 slice；
         */

        let arr01: [i32; 5] = [1, 2, 3, 4, 5];

        // 使用 &[i32] 表示 i32 类型的数组的 slice；
        let slice01: &[i32] = &arr01[..=2];
    }
}
