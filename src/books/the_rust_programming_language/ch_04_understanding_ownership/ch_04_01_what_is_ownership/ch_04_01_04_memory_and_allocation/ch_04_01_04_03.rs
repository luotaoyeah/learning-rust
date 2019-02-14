//
// Understanding Ownership
//     Memory and Allocation
//         Ways Variables and Data Interact: Move
//

pub fn fn_04_01_04_03() {
    println!("-------------------------------------------------- 01");
    {
        // 对于 String 类型的数据，在内存中保存了两份数据，一份在 stack 上，一份在 heap 上，
        // 在 stack 上的是数据的元信息，包括：
        //     ptr 实际数据的内存地址
        //     len 当前数据的大小
        //     capacity 分配的内存大小
        // 在 heap 上的是数据的实际内容

        // 当把一个 String 变量赋值给另外一个 String 变量时，会复制一份 stack 上的元信息，
        // 其 ptr 指向同样一份实际数据，
        // 且原来的那一份元信息（原来的变量）会失效，不能再被访问，
        // 这个操作称之为 move 操作

        // move 操作解决的问题是：确保数据在同一时间只会有一个 owner，
        // 保证了内存的正确释放

        // 如下，将 s01 赋值给 s02 之后，s01 将失效，不能再被访问
        let s01: String = String::from("FOO");
        let s02: String = s01;

        println!("{}", s02); // FOO
        /*
                println!("{}", s01); // [E0382]: borrow of moved value: `s01`
        */
    }
}
