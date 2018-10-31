/*
   Common Collections
       Vectors
           Updating a New Vector
               Listing 8-3: Using the push method to add values to a vector

 */

pub fn fn_08_01_02_01() {
    println!("-------------------------------------------------- 01");
    {
        let mut vec01: Vec<i32> = Vec::new();

        /*
           使用 push() 方法，往 vector 中插入数据；
         */
        vec01.push(1);
        vec01.push(2);
        vec01.push(3);

        println!("{:?}", vec01); // [1, 2, 3]
    }
}
