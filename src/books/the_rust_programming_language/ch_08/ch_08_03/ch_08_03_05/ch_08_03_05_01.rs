/*
   Common Collections
       Summary
 */

use std::collections::HashMap;

/*
   Given a list of integers,
   use a vector and return the mean (the average value),
   median (when sorted, the value in the middle position),
   mode (the value that occurs most often; a hash map will be helpful here) of the list.
 */
pub fn fn_08_03_05_01() {
    let mut array01: [i32; 6] = [3, 3, 3, 1, 2, 2];

    println!("-------------------------------------------------- 01");
    {
        /*
           求平均数
         */

        // 求和
        let mut sum: i32 = 0;
        for (_i, &item) in array01.iter().enumerate() {
            sum += item;
        }
        // 取长度
        let len: i32 = array01.len() as i32;

        println!("{}", (sum as f32) / (len as f32)); // 2.3333333
    }

    println!("-------------------------------------------------- 02");
    {
        /*
           求中间值
         */

        // 排序
        array01.sort();
        // 取中间值的索引
        let index = array01.len() / 2;
        // 取中间值
        let median = array01.get(index);

        match median {
            Some(i) => {
                println!("{}", i); // 3
            }
            None => (),
        }
    }

    println!("-------------------------------------------------- 03");
    {
        /*
           查找出现次数最多的数字
         */

        let mut map: HashMap<i32, i32> = HashMap::new();

        for (_i, &item) in array01.iter().enumerate() {
            let count = map.entry(item).or_insert(0);
            *count += 1;
        }

        let mut max: i32 = 0;

        for (k, v) in map {
            if v > max {
                max = k;
            }
        }

        println!("{}", max);
    }
}
