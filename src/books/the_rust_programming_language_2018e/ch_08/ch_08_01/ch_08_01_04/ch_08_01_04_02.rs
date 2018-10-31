/*
   Common Collections
       Vectors
           Reading Elements of Vectors

 */

pub fn fn_08_01_04_02() {
    println!("-------------------------------------------------- 01");
    {
        /*
           使用 index 访问 vector 成员，如果 index 超出范围，程序会 panic；
         */

        let vec: Vec<i32> = vec![1, 2, 3];
        /*
                println!("{}", &vec[3]); // thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 3'
        */
    }

    println!("-------------------------------------------------- 02");
    {
        /*
           使用 get() 访问 vector 成员，如果 index 超出范围，会返回 None；
         */

        let vec: Vec<i32> = vec![1, 2, 3];
        println!("{:?}", vec.get(3)); // None
    }
}
