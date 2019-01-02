/*
  Functional Language Features: Iterators and Closures
      Processing a Series of Items with Iterators
*/

pub fn fn_13_02_01_01() {
    println!("-------------------------------------------------- 01");
    {
        let vec = vec!["a", "b", "c"];

        let iter = vec.iter();

        for item in iter {
            println!("{}", item);
        }
    }
}
