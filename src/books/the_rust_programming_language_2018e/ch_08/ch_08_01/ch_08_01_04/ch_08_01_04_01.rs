/*
   Common Collections
       Vectors
           Reading Elements of Vectors

 */

pub fn fn_08_01_04_01() {
    println!("-------------------------------------------------- 01");
    {
        let vec: Vec<i32> = vec![1, 2, 3, 4, 5];

        /*
           可以使用 index 访问 vector 的成员，
           返回的类型为 &T
         */
        let i: &i32 = &vec[1];
        println!("{}", i); // 2

        /*
           可以使用 get() 方法访问 vector 的成员，
           返回的类型为欸 Option<&T>
         */
        let j: Option<&i32> = vec.get(1);
        println!("{:?}", j); // Some(2)
    }
}
