//
// Understanding Ownership
//     Memory and Allocation
//         Ways Variables and Data Interact: Clone
//

pub fn fn_04_01_04_04() {
    println!("-------------------------------------------------- 01");
    {
        // 如果确实想要复制 heap 上的实际数据，而不仅仅是 stack 上的元信息，
        // 可以使用 clone() 方法

        let s01: String = String::from("FOO");
        let s02: String = s01.clone();

        println!("{}", s01); // FOO
        println!("{}", s02); // FOO

        println!("{}", s01 == s02); // true
        assert_eq!(s01, s02);
    }

    println!("-------------------------------------------------- 02");
    {
        // 也可以使用 clone_from() 方法，
        // 将一个 String 的数据克隆到另外一个以及存在的 String 变量里面
        let s01: String = String::from("FOO");
        let mut s02: String = String::from("BAR");
        s02.clone_from(&s01);

        println!("{}", s01); // FOO
        println!("{}", s02); // FOO

        println!("{}", s01 == s02); // true
        assert_eq!(s01, s02);
    }
}
