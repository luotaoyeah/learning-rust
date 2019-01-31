/*
  Naming Objects
      3.11. Using the Functions of the Standard Library
          some str methods
*/

pub fn fn_03_11_02() {
    println!("-------------------------------------------------- 01");

    let s01: &str = "rust";

    println!("{} {}", s01.contains("u"), s01.contains("x")); // true false
    println!("{} {}", s01.starts_with("r"), s01.starts_with("x")); // true false
    println!("{} {}", s01.ends_with("t"), s01.ends_with("x")); // true false
    println!("{:?}", s01.as_bytes()); // [114, 117, 115, 116]
}
