/*
  Smart Pointers
      Reference Cycles Can Leak Memory
          Preventing Reference Cycles: Turning an Rc<T> into a Weak<T>
              Creating a Tree Data Structure: a Node with Child Nodes
              Adding a Reference from a Child to Its Parent

*/

use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;

pub fn fn_15_06_02_01() {
    println!("-------------------------------------------------- 01");
    {
        /*
          使用 Rc::clone() 获取 Rc 实例的一个 ownership，
              同时 strong_count 加一；
          使用 Rc::downgrade() 获取 Rc 实例的一个 weak reference，
              同时 weak_count 加一；
          使用 upgrade() 方法，返回一个 Option<Rc<T>>，表示引用的 Rc 实例是否可用；
        */

        #[derive(Debug)]
        struct Node {
            value: i32,
            parent: RefCell<Weak<Node>>,
            children: RefCell<Vec<Rc<Node>>>,
        }

        let leaf = Rc::new(Node {
            value: 3,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        });

        println!("{:?}", leaf.parent.borrow().upgrade());

        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!("{:#?}", leaf.parent.borrow().upgrade());
    }
}
