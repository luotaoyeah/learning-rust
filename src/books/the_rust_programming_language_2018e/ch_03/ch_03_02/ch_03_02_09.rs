/*
   Common Programming Concepts
       Data Types
           Compound Types
               The Array Type
 */

pub fn fn_03_02_09() {
    println!("-------------------------------------------------- 01");
    {
        /*
           array 用来表示多个值，多个元素之间使用逗号（,）分隔，多个元素使用方括号（[]）包裹，
           array 中元素的类型必须相同，
           array 的长度是固定的，不能增加或者移除元素；
         */
        let arr01 = [1, 2, 3];

        /*
           array 的类型使用 [type; number] 的形式来表示，
           type 表示元素的类型，number 表示 array 的长度；
         */
        let arr02: [i32; 5] = [1, 2, 3, 4, 5];
    }
}
