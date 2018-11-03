/*
   Generic Types, Traits，Lifetimes
       Removing Duplication by Extracting a Function
 */

pub fn fn_10_01_01_01() {
    println!("-------------------------------------------------- 01");
    {
        /*
            查找最大值
         */

        let number_list: Vec<i32> = vec![34, 50, 25, 100, 65];

        let mut max: i32 = number_list[0];

        for n in number_list {
            if n > max {
                max = n;
            }
        }

        println!("{}", max); // 100
    }

    println!("-------------------------------------------------- 02");
    {
        let number_list: Vec<i32> = vec![102, 34, 6000, 89, 54, 2, 43, 8];
        println!("{}", max(&number_list));
    }
}

/// 查找最大值
fn max(list: &[i32]) -> i32 {
    let mut max: i32 = list[0];

    for &n in list {
        if n > max {
            max = n;
        }
    }

    max
}
