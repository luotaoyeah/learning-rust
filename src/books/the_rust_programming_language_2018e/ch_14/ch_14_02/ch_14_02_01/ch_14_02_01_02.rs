/*
  More About Cargo and Crates.io
      Publishing a Crate to Crates.io
          Documentation Comments as Tests
*/

pub fn fn_14_01_02_02() {
    println!("-------------------------------------------------- 01");
    {
        add_one(4);
    }
}

/// 加一
///
/// # Examples
///
/// ```
/// let i = 5;
/// assert_eq!(6, add_one(i));
/// ```
///
pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
