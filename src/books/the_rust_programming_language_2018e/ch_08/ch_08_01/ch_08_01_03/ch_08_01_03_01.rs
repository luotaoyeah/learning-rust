/*
   Common Collections
       Vectors
           Dropping a Vector Drops Its Elements
               Listing 8-4: Showing where the vector and its elements are dropped

 */

pub fn fn_08_01_03_01() {
    println!("-------------------------------------------------- 01");
    {
        let vec01: Vec<i32> = vec![1, 2, 3];
        /*
           当 vector 离开 scope 时，会被 drop 掉，
           同时它的元素也会被 drop 掉；
         */
    }
}
