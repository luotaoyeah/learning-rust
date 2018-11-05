/*
   Writing Automated Tests
       Test Organization
           Unit Tests
               Testing Private Functions
 */

pub fn fn_11_03_01_02() {
    println!("-------------------------------------------------- 01");
    {}
}

fn private_fn() -> i32 {
    println!("private function");
    9
}

#[cfg(test)]
mod tests {
    use super::*;

    /*
       可以测试私有方法
     */
    #[test]
    fn test_01() {
        assert_eq!(private_fn(), 9);
    }
}
