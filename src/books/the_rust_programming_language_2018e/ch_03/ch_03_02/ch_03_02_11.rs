/*
   Common Programming Concepts
       Data Types
           Compound Types
               The Array Type
                   Invalid Array Element Access
 */

pub fn fn_03_02_11() {
    println!("-------------------------------------------------- 01");
    {
        let arr01: [char; 3] = ['a', 'b', 'c'];

        /*
           如果访问的数组下标超过了数组的长度，且下标在编译期可以确定，
           则会报编译时错误；
         */
        println!("{}", arr01[10]); // error: index out of bounds: the len is 3 but the index is 10
    }

    println!("-------------------------------------------------- 02");
    {
        let arr01: [char; 3] = ['a', 'b', 'c'];

        let i = 10;
        /*
           如果访问的数组下标超过了数组的长度，且下标在编译期不能确定，
           则会报运行时错误；
         */
        println!("{}", arr01[i]); // thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 10'
    }
}
