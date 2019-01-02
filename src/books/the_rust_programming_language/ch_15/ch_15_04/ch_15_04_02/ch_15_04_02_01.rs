/*
  Smart Pointers
      Rc<T>, the Reference Counted Smart Pointer
          Cloning an Rc<T> Increases the Reference Count
*/

use std::rc::Rc;

pub fn fn_15_04_02_01() {
    println!("-------------------------------------------------- 01");
    {
        enum List {
            Cons(i32, Rc<List>),
            Nil,
        }

        let a = Rc::new(List::Cons(5, Rc::new(List::Cons(10, Rc::new(List::Nil)))));
        println!("create a: {}", Rc::strong_count(&a)); // create a: 1
        let b = List::Cons(3, Rc::clone(&a));
        println!("create b: {}", Rc::strong_count(&a)); // create b: 2
        {
            let c = List::Cons(4, Rc::clone(&a));
            println!("create c: {}", Rc::strong_count(&a)); // create c: 3
        }
        println!("drop c: {}", Rc::strong_count(&a)); // drop c: 2
    }
}
