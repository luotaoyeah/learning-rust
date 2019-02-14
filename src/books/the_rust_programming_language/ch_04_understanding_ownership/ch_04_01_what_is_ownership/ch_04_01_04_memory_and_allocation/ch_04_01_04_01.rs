/*
  Understanding Ownership
      Memory and Allocation
*/

pub fn fn_04_01_04_01() {
    println!("-------------------------------------------------- 01");
    {
        // string literal 的内容在编译时是已知的，且不可更改，因此它是硬编码在可执行程序中的，
        // 而 String 类型的数据，其内容和大小都是可以改变的，因此是存放在 heap 中的，
        // 需要在运行时期向操作系统申请内存，同时要有一定的机制在适当的时候来释放内存

        // 对于有 GC 的语言（比如java）来说，释放内存这个过程是由 GC 来完成的
        // 对于没有 GC 的语言来说，释放内存这个过程必须由我们手动来完成，
        // 而对于 rust 来说，释放内存这个过程会自动执行，执行的时机是：当数据的 owner 离开作用域时，
        // 具体方式是：rust 会自动调用该类型的 drop() 方法，在这个方法中实现内存的释放

        let s01 = String::from("foo");
        println!("{}", s01); // foo

        let mut s02 = String::from(s01);
        println!("{}", s02); // foo

        String::push_str(&mut s02, " bar");
        println!("{}", s02); // foo bar
    }
}
