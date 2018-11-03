/*
   Error Handling
       Recoverable Errors with `Result`
           Propagating Errors
               The `?` Operator Can Only Be Used in Functions that Return `Result`

 */

use std::fs::File;

pub fn fn_09_02_06_01() {
    println!("-------------------------------------------------- 01");
    {
        // [E0277]: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `std::ops::Try`)
        File::open("foo.txt")?;
    }
}
