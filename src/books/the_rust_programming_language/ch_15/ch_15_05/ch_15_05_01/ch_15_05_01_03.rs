/*
  Smart Pointers
      RefCell<T> and the Interior Mutability Pattern
          Having Multiple Owners of Mutable Data by Combining Rc<T> and RefCell<T>

*/

use std::cell::RefCell;
use std::rc::Rc;

pub fn fn_15_05_01_03() {
    println!("-------------------------------------------------- 01");
    {
        /*
          将 Rc<T> 和 RefCell<T> 结合，可以通过拥有多个 mutable owner；
        */

        #[derive(Debug)]
        enum List {
            Cons(Rc<RefCell<i32>>, Rc<List>),
            Nil,
        }

        let value = Rc::new(RefCell::new(5));

        let a = Rc::new(List::Cons(Rc::clone(&value), Rc::new(List::Nil)));

        let b = List::Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
        let c = List::Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

        *value.borrow_mut() = 10;

        println!("{:?}", a);
        println!("{:?}", b);
        println!("{:?}", c);
    }
}
