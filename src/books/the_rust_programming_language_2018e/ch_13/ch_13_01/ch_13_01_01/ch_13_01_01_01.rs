/*
  Functional Language Features: Iterators and Closures
      Closures: Anonymous Functions that Can Capture Their Environment
          Creating an Abstraction of Behavior with Closures
              Refactoring Using Functions
              Refactoring with Closures to Store Code
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
        generate_workout_03(30, 0);
    }
}

/// 模拟耗时计算
fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

///
fn generate_workout_01(itensity: u32, random_num: u32) {
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

/// refactoring with function
fn generate_workout_02(itensity: u32, random_num: u32) {
    let result = simulated_expensive_calculation(itensity);

    if itensity < 25 {
        println!("{} pushups", result);
        println!("{} situps", result);
    } else {
        if random_num == 3 {
            println!("take a break");
        } else {
            println!("run for {} minutes", result);
        }
    }
}

/// refactoring with closure
fn generate_workout_03(itensity: u32, random_num: u32) {
    let expensive_closure = |itensity| {
        println!("calculating...");
        thread::sleep(Duration::from_secs(2));
        itensity
    };

    if itensity < 25 {
        println!("{} pushups", expensive_closure(itensity));
        println!("{} situps", expensive_closure(itensity));
    } else {
        if random_num == 3 {
            println!("take a break");
        } else {
            println!("run for {} minutes", expensive_closure(itensity));
        }
    }
}
