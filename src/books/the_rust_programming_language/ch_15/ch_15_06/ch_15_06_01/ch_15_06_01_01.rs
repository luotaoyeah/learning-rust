/*
  Smart Pointers
      Reference Cycles Can Leak Memory
          Creating a Reference Cycle

*/

use std::cell::RefCell;
use std::rc::Rc;

pub fn fn_15_06_01_01() {
    println!("-------------------------------------------------- 01");
    {
        /*
          循环引用会导致内存泄漏；
        */

        #[derive(Debug)]
        enum List {
            Cons(i32, RefCell<Rc<List>>),
            Nil,
        }

        impl List {
            fn tail(&self) -> Option<&RefCell<Rc<List>>> {
                match self {
                    List::Cons(_, list) => Some(list),
                    List::Nil => None,
                }
            }
        }

        let a = Rc::new(List::Cons(5, RefCell::new(Rc::new(List::Nil))));
        let b = Rc::new(List::Cons(10, RefCell::new(Rc::clone(&a))));

        if let Some(list) = a.tail() {
            *list.borrow_mut() = Rc::clone(&b);
        }

        println!("{}", Rc::strong_count(&a)); // 2
        println!("{}", Rc::strong_count(&b)); // 2
    }
}
