/*
  Smart Pointers
      Rc<T>, the Reference Counted Smart Pointer
          Using Rc<T> to Share Data
*/

use std::rc::Rc;

pub fn fn_15_04_01_01() {
    println!("-------------------------------------------------- 01");
    {
        /*
          Rc<T> （Rc 表示 reference counting）用来表示多个 reference 指向同一个数据；
          因为在 compile 时期，不能确定哪一个 reference 引用该数据的时间最长，
          因此不能指定该数据的唯一 owner；
        
          Rc<T> 只能在单线程场景下使用；
        */

        enum List {
            Cons(i32, Box<List>),
            Nil,
        }

        let a = List::Cons(5, Box::new(List::Cons(10, Box::new(List::Nil))));
        let b = List::Cons(3, Box::new(a));
        /*
                let c = List::Cons(4, Box::new(a)); // [E0382]: use of moved value: `a`
        */
    }

    println!("-------------------------------------------------- 02");
    {
        enum List {
            Cons(i32, Rc<List>),
            Nil,
        }

        let a = Rc::new(List::Cons(5, Rc::new(List::Cons(10, Rc::new(List::Nil)))));
        /*
          调用 Rc::clone() 方法，引用计数加一；
        */
        let b = List::Cons(3, Rc::clone(&a));
        let c = List::Cons(4, Rc::clone(&a));
    }
}
