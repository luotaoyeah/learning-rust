/*
   Common Collections
       Vectors
           Creating a New Vector
               Listing 8-1: Creating a new, empty vector to hold values of type i32

 */

pub fn fn_08_01_01_01() {
    println!("-------------------------------------------------- 01");
    {
        /*
           使用向量（Vec<T>）在内存中存储连续数据，数据的类型必须相同；
         */

        // 使用 Vec::new() 创建一个空的 vector，此时必须标注类型；
        let vec01: Vec<i32> = Vec::new();
        println!("{:?}", vec01); // []

        // 使用宏 vec! 创建一个 vector，并进行初始化，此时可以推断类型；
        let vec02 = vec![1, 2, 3];
        println!("{:?}", vec02); // [1, 2, 3]
    }
}
