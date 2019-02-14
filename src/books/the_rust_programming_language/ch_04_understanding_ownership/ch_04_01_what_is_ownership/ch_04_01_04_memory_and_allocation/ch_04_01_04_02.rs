//
// Understanding Ownership
//     Memory and Allocation
//         Ways Variables and Data Interact: Move
//

pub fn fn_04_01_04_02() {
    println!("-------------------------------------------------- 01");
    {
        // integer 是简单类型，数据是存放在 stack 上的，
        // 当把一个 integer 变量赋值给另外一个 integer 变量时，会在 stack 上复制一份数据

        let i01: i32 = 9;
        let i02: i32 = i01;


        println!("{}", i01); // 9
        println!("{}", i02); // 9
    }
}
