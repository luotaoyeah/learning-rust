/*
  Functions
      Functions
        fn
*/

pub fn fn_03_03_01_01() {
    println!("-------------------------------------------------- 01");
    another_fn(); // FOO
}

// 使用关键字 fn 定义一个函数
// 函数名称使用 snake_case 命名规范
fn another_fn() {
    println!("{}", "FOO");
}
