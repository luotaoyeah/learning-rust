/*
   Common Programming Concepts
       Data Types
           Compound Types
               The Array Type
                   Accessing Array Elements
 */

pub fn fn_03_02_10() {
    println!("-------------------------------------------------- 01");
    {
        let arr01: [char; 3] = ['a', 'b', 'c'];

        // 通过下标索引，访问数组的各个元素
        println!("{}, {}, {}", arr01[0], arr01[1], arr01[2]); // a, b, c
    }
}
