/*
   Enums and Pattern Matching
       match
           Matches are Exhaustive
 */

pub fn fn_06_02_04_01() {
    println!("-------------------------------------------------- 01");
    {
        /*
           match 的 arms 必须包含 enum 的所有 variants，否则编译报错；
         */

        fn plus_one(x: Option<i32>) -> Option<i32> {
            // [E0004]: non-exhaustive patterns: `None` not covered
            match x {
                Some(i) => Some(i + 1),
            }
        }
    }
}
