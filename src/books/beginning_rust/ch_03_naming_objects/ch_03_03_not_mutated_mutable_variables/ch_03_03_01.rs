/*
  Naming Objects
      3.3. Not Mutated Mutable Variables
*/

pub fn fn_03_03_01() {
    println!("-------------------------------------------------- 01");
    // 如果声明了一个 mutable variable，但是并没有对它进行重新赋值，
    // 那么编译会报警告

    // warning: variable does not need to be mutable
    let mut i: u32 = 9;
    println!("{}", i);
}
