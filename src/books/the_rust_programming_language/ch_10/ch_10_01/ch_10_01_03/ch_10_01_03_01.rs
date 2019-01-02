/*
   Generic Types, Traits，Lifetimes
       Generic Data Types
           In Enum Definitions
 */

pub fn fn_10_01_03_01() {
    println!("-------------------------------------------------- 01");
    {
        enum Option<T> {
            Some(T),
            None,
        }

        enum Result<T, E> {
            Ok(T),
            Err(E),
        }
    }
}
