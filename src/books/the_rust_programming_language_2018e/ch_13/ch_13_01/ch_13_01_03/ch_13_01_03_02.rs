/*
  Functional Language Features: Iterators and Closures
      Closures: Anonymous Functions that Can Capture Their Environment
          Limitations of the Cacher Implementation
*/

use std::collections::HashMap;

pub fn fn_13_01_03_02() {
    println!("-------------------------------------------------- 01");
    {}
}

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: HashMap<u32, u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        if self.value.contains_key(&arg) {
            *self.value.get(&arg).unwrap()
        } else {
            let v = (self.calculation)(arg);
            self.value.insert(arg, v);
            v
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_01() {
        let mut cacher = Cacher::new(|x| x);

        let v01 = cacher.value(1);
        let v02 = cacher.value(2);

        assert_eq!(2, v02);
    }
}
