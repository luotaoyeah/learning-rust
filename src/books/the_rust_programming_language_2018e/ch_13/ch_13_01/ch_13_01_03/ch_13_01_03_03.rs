/*
  Functional Language Features: Iterators and Closures
      Closures: Anonymous Functions that Can Capture Their Environment
          Limitations of the Cacher Implementation
*/

use std::collections::HashMap;
use std::hash::Hash;

pub fn fn_13_01_03_03() {
    println!("-------------------------------------------------- 01");
    {}
}

///
struct Cacher<C, K, V>
where
    C: Fn(K) -> V,
    K: Copy + Hash + Eq,
    V: Copy,
{
    calculation: C,
    value: HashMap<K, V>,
}

///
impl<C, K, V> Cacher<C, K, V>
where
    C: Fn(K) -> V,
    K: Copy + Hash + Eq,
    V: Copy,
{
    fn new(calculation: C) -> Cacher<C, K, V> {
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }

    fn value(&mut self, arg: K) -> V {
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
        assert_eq!(1, v01);
    }

    #[test]
    fn test_02() {
        let mut cacher = Cacher::new(|x| x);
        let v01 = cacher.value("foo");
        assert_eq!("foo", v01);
    }
}
