/*
   Enums and Pattern Matching
       match
           Matching with Option<T>
 */

pub fn fn_06_02_03_01() {
    println!("-------------------------------------------------- 01");
    {
        ///
        fn plus_one(x: Option<i32>) -> Option<i32> {
            match x {
                None => None,
                Some(i) => Some(i + 1),
            }
        }

        println!("{:?}", plus_one(Some(5))); // Some(6)
        println!("{:?}", plus_one(None)); // None
    }
}
