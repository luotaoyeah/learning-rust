/*
  Smart Pointers
      Treating Smart Pointers Like Regular References with the Deref Trait
          Using Box<T> Like a Reference
*/

pub fn fn_15_02_02_01() {
    println!("-------------------------------------------------- 01");
    {
        let x = 5;
        let y = Box::new(x);

        assert_eq!(5, x);
        assert_eq!(5, *y);
    }
}
