/*
   Common Collections
       Vectors
           Reading Elements of Vectors

 */

pub fn fn_08_01_04_03() {
    println!("-------------------------------------------------- 01");
    {
        /*
            无论使用 index 方式，还是 get() 方式访问成员，
            返回的都是一个 reference（None 除外），因此遵循 ownership rules 和 borrowing rules；
            比如：在一个 scope 中不能同时存在 mutable inference 和 immutable reference；
         */

        let mut vec: Vec<i32> = vec![1, 2, 3];

        // immutable borrow
        let i: &i32 = &vec[1];

        // mutable borrow
        vec.push(4); // [E0502]: cannot borrow `vec` as mutable because it is also borrowed as immutable
    }
}
