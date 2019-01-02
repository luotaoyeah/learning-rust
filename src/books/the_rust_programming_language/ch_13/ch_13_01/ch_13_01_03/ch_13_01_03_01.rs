/*
  Functional Language Features: Iterators and Closures
      Closures: Anonymous Functions that Can Capture Their Environment
          Storing Closures Using Generic Parameters and the Fn Traits
*/

use std::thread;
use std::time::Duration;

pub fn fn_13_01_03_01() {
    println!("-------------------------------------------------- 01");
    {
        /*
          每个 closure 的类型都是唯一的，
          即使两个 closure 的签名一样，它们也是不同的类型；
          每个 closure 至少实现这三个 trait 中的一个：Fn，FnOnce，FnMut；
        */

        generate_workout(20, 3);
    }
}

/// T 是一个 closure
struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

///
fn generate_workout(itensity: u32, random_num: u32) {
    let mut expensive_closure = Cacher::new(|itensity| {
        println!("calculating...");
        thread::sleep(Duration::from_secs(2));
        itensity
    });

    if itensity < 25 {
        println!("{} pushups", expensive_closure.value(itensity));
        println!("{} situps", expensive_closure.value(itensity));
    } else {
        if random_num == 3 {
            println!("take a break");
        } else {
            println!("{} minutes running", expensive_closure.value(itensity));
        }
    }
}
