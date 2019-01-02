/*
  Smart Pointers
      Treating Smart Pointers Like Regular References with the Deref Trait
          Following the Pointer to the Value with the Dereference Operator
*/

pub fn fn_15_02_01_01() {
    println!("-------------------------------------------------- 01");
    {
        let x = 5;
        let y = &x;

        assert_eq!(x, 5);
        assert_eq!(*y, 5);
    }
}
