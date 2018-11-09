/*
  Functional Language Features: Iterators and Closures
      Processing a Series of Items with Iterators
          Creating Our Own Iterators with the Iterator Trait
          Using Our Counter Iterator’s next Method
*/

pub fn fn_13_02_06_01() {
    println!("-------------------------------------------------- 01");
    {
        /*
        
        */

        struct Counter {
            count: u32,
        }

        impl Counter {
            fn new() -> Counter {
                Counter { count: 0 }
            }
        }

        impl Iterator for Counter {
            type Item = u32;

            fn next(&mut self) -> Option<<Self as Iterator>::Item> {
                self.count += 1;

                if self.count < 6 {
                    Some(self.count)
                } else {
                    None
                }
            }
        }

        let sum = Counter::new()
            .zip(Counter::new().skip(1))
            .map(|(a, b)| a * b)
            .filter(|i| i % 3 == 0)
            .sum::<u32>();

        assert_eq!(sum, 18);
    }
}
