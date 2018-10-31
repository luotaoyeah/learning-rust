/*
   Common Collections
       Vectors
           Iterating over the Values in a Vector

 */

pub fn fn_08_01_05_01() {
    println!("-------------------------------------------------- 01");
    {
        let vec: Vec<&str> = vec!["a", "b", "c"];

        /*
           使用 for..in 遍历 vector 的元素；
         */

        for i in &vec {
            print!("{}", i); // abc
        }
        println!();
    }

    println!("-------------------------------------------------- 02");
    {
        let mut vec: Vec<&str> = vec!["a", "b", "c"];

        /*
           遍历的时候修改元素的值；
           使用 dereference operator（*）访问 reference 所指向的数据；
         */
        for i in &mut vec {
            *i = "x";
        }

        println!("{:?}", &vec); // ["x", "x", "x"]
    }
}
