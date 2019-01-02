/*
   Generic Types, Traits，Lifetimes
       Traits
           Fixing the largest Function with Trait Bounds
 */

pub fn fn_10_02_04_03() {
    println!("-------------------------------------------------- 01");
    {
        let list01: Vec<i32> = vec![1, 2, 3, 4, 5];
        println!("{}", max_01(&list01));

        let list02: Vec<char> = vec!['a', 'b', 'c'];
        println!("{}", max_01(&list02));
    }
}

fn max_01<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut max: T = list[0];

    for &n in list.iter() {
        if n > max {
            max = n;
        }
    }

    max
}
