/*
  Smart Pointers
      RefCell<T> and the Interior Mutability Pattern
          Enforcing Borrowing Rules at Runtime with RefCell<T>
          Interior Mutability: A Mutable Borrow to an Immutable Value
              Keeping Track of Borrows at Runtime with RefCell<T>

*/

use std::rc::Rc;

pub fn fn_15_05_01_02() {
    println!("-------------------------------------------------- 01");
    {
        /*
          对于 reference，使用 & 获取 immutable reference，
          使用 &mut 获取 mutable inference，
          同时最多只能有一个 immutable reference；
        
          对于 RefCell<T>，使用 borrow() 获取 immutable Ref<T>，
          使用 borrow_mut() 获取 mutable RefMut<T>，
          同时最多只能有一个 mutable RefMut<T>；
          Ref<T> 和 RefMut<T> 都实现了 Deref 特性；
        */
    }
}

pub trait Messager {
    fn send(&self, msg: &str);
}

pub struct Tracker<'a, T>
where
    T: 'a + Messager,
{
    messager: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> Tracker<'a, T>
where
    T: Messager,
{
    fn new(messager: &T, max: usize) -> Tracker<T> {
        Tracker {
            messager,
            max,
            value: 0,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage = value as f64 / self.max as f64;

        if percentage >= 0.75 && percentage < 0.9 {
            self.messager.send("[0.75, 0.9)");
        } else if percentage >= 0.9 {
            self.messager.send("[0.9)");
        } else if percentage >= 1.0 {
            self.messager.send("[1)");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessager {
        messages: RefCell<Vec<String>>,
    }

    impl MockMessager {
        fn new() -> MockMessager {
            MockMessager {
                messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messager for MockMessager {
        fn send(&self, msg: &str) {
            self.messages.borrow_mut().push(String::from(msg));

            /*
              同时最多只能有一个 mutable RefMut<T>；
            */
            let x = self.messages.borrow_mut();
            let y = self.messages.borrow_mut();
        }
    }

    #[test]
    fn test() {
        let messager = MockMessager::new();
        let mut tracker = Tracker::new(&messager, 100);
        tracker.set_value(80);

        assert_eq!(messager.messages.borrow().len(), 1);
    }
}
