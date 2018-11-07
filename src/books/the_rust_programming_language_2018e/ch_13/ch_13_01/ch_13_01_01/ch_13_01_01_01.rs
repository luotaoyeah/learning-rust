/*
  Functional Language Features: Iterators and Closures
      Closures: Anonymous Functions that Can Capture Their Environment
          Creating an Abstraction of Behavior with Closures
*/

use std::thread;
use std::time::Duration;

/*
  rust 中的 closure 是 anonymous function，
  closure 可以被赋值给变量，可以作为参数传递给其他函数；
  closure 可以捕获变量；
*/
pub fn fn_13_01_01_01() {
    println!("-------------------------------------------------- 01");
    {
        let itensity: u32 = 10;
        let random_num: u32 = 7;

        generate_workout(itensity, random_num);
    }
}

/// 模拟耗时计算
fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

///
fn generate_workout(itensity: u32, random_num: u32) {
    if itensity < 25 {
        println!("{} pushups", simulated_expensive_calculation(itensity));
        println!("{} situps", simulated_expensive_calculation(itensity));
    } else {
        if random_num == 3 {
            println!("take a break");
        } else {
            println!(
                "run for {} minutes",
                simulated_expensive_calculation(itensity)
            );
        }
    }
}
