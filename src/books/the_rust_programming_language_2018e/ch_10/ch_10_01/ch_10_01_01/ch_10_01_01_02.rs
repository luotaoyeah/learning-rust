/*
   Generic Types, Traits，Lifetimes
       Generic Data Types
           In Function Definitions
 */

pub fn fn_10_01_01_02() {
    println!("-------------------------------------------------- 01");
    {
        let list01: Vec<i32> = vec![34, 50, 25, 100, 65];
        println!("{}", max_i32(&list01));

        let list02: Vec<char> = vec!['y', 'm', 'a', 'q'];
        println!("{}", max_char(&list02));
    }
}

///
fn max_i32(list: &[i32]) -> i32 {
    let mut max = list[0];

    for &n in list.iter() {
        if n > max {
            max = n;
        }
    }

    max
}

///
fn max_char(list: &[char]) -> char {
    let mut max: char = list[0];

    for &n in list.iter() {
        if n > max {
            max = n;
        }
    }

    max
}

///
fn max<T>(list: &[T]) -> T {
    let mut max: T = list[0];

    for &n in list.iter() {
        if n > max {
            max = n;
        }
    }

    max
}
