/*
  Smart Pointers
      Treating Smart Pointers Like Regular References with the Deref Trait
          Defining Our Own Smart Pointer
*/

pub fn fn_15_02_02_02() {
    println!("-------------------------------------------------- 01");
    {
        struct MyBox<T>(T);
        impl<T> MyBox<T> {
            fn new(x: T) -> MyBox<T> {
                MyBox(x)
            }
        }

        let x = 5;
        let y = MyBox::new(x);
        assert_eq!(5, x);
        assert_eq!(5, *y); // [E0614]: type `MyBox<{integer}>` cannot be dereferenced
    }
}
