/*
   Understanding Ownership
       Memory and Allocation
           Ways Variables and Data Interact: Move
 */

/*
   在 rust 中，多个 variables 可以跟同一份 data 交互；
 */
pub fn fn_04_01_05() {
    println!("-------------------------------------------------- 01");
    {
        /*
           因为 5 是简单类型，在 compile 时期可以知道它的内容和长度，
           因此 5 是存储在 stack 上的；
           当把 x 赋值给 y 时，rust 会复制一个 5，然后存储在 stack 上，然后赋值给 y；
           所以这儿发生的是 copy；
         */
        let x: i32 = 5;
        let y: i32 = x;

        println!("{}", y);
    }

    println!("-------------------------------------------------- 02");
    {
        /*
           对于 String 类型，在内存中会存储两份信息：
               1. 一份信息保存在 stack 上，包括：
                      ptr：指针，指向 heap 上保存实际数据的内存地址
                      length：当前使用的内存大小（bytes）
                      capacity：总共分配的内存大小（bytes）
               2. 一份信息保存在 heap 上，保存的时实际数据；
         */

        /*
           当把 s01 赋值给 s02 时，根据以往的经验，有可能会发生如下的情况：
               在内存中只会复制 s01 在 stack 上的信息，即（ptr，length，capacity），
               而 heap 上的实际数据不会复制；
           而实际上在 rust 中，当把 s01 赋值给 s02 时，s01 会被认为已经失效，
           这儿发生的操作称之为 move；
         */
        let s01 = String::from("hello");
        let s02 = s01;
        println!("{}", s02);
        println!("{}", s01); // [E0382]: use of moved value: `s01`
    }
}
