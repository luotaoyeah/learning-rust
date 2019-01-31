//
// Understanding Ownership
//     What Is Ownership?
//     Ownership Rules
//

//
// ownership 是 rust 的核心特性
// 程序管理内存一般有两种方式：
//     1. GC
//     2. 手动分配，手动释放
// rust 的内存管理属于第三种方式：使用 ownership 管理
//

//
// ownership 定义的三个规则：
//     1. 每一个数据（value）都有一个对应的变量（variable），称之为它的 owner
//     2. 每一个数据在任何时候只能有一个 owner
//     3. 当 owner 离开作用域（scope）时，value 就会被丢弃（drop）
//
pub fn fn_04_01_01() {
    println!("-------------------------------------------------- 01");
    {}
}
