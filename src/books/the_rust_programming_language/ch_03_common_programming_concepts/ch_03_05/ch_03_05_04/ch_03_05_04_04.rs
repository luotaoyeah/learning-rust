/*
   Control Flow
       Repetition with Loops
           Looping Through a Collection with for
 */

pub fn fn_03_05_04_04() {
    let arr: [i32; 3] = [10, 20, 30];

    println!("-------------------------------------------------- 01");
    {
        /*
          使用 while 遍历数组元素；
         */
        let mut i: usize = 0;
        while i < arr.len() {
            println!("{}", arr[i]);
            i += 1;
        }
    }

    println!("-------------------------------------------------- 02");
    {
        /*
          使用 loop 遍历数组元素；
         */
        let mut i: usize = 0;
        loop {
            if i >= arr.len() {
                break;
            }

            println!("{}", arr[i]);
            i += 1;
        }
    }

    println!("-------------------------------------------------- 03");
    {
        /*
          使用 for 遍历数组元素；
         */

        for el in arr.iter() {
            println!("{}", el);
        }
    }
}
