/*
   Understanding Ownership
       Memory and Allocation
 */

/*
   因为在 compile 时期可以知道 string literal 的内容和长度，因此 string literal 是硬编码在程序代码中的；
   而对于 String 来说，在 compile 时期是不知道内容和长度的，需要在 runtime 时期向操作系统申请内存分配，
   同时在不再需要时，让操作系统回收内存；
   由于 rust 没有 GC，所以我们必须手动释放内存；
       如果忘记释放，会造成内存泄漏，
       如果太早释放，会造成程序错误，
       对于一次内存申请，必须刚好对应一次内存释放；
   rust 的 ownership 实现的是：当 value 的 owner 离开 scope 时，对应的内存会自动释放，
   如何实现：rust 自动调用一个特殊的方法 drop()；
 */

/*
   C++ 中类似的模式为：RAII（Resource Acquisition Is Initialization）；
 */
pub fn fn_04_01_04() {
    println!("-------------------------------------------------- 01");
    {
        // 通过调用 String::from() 方法，向操作系统申请内存；
        let mut s = String::from("hello");
        s.push_str(" world");
        println!("{}", s);
    }
}
