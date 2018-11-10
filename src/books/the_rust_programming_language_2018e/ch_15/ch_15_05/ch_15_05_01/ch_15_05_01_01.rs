/*
  Smart Pointers
      RefCell<T> and the Interior Mutability Pattern
          Enforcing Borrowing Rules at Runtime with RefCell<T>
          Interior Mutability: A Mutable Borrow to an Immutable Value
              A Use Case for Interior Mutability: Mock Objects

*/

use std::rc::Rc;

pub fn fn_15_05_01_01() {
    println!("-------------------------------------------------- 01");
    {
        /*
          interior mutability 是 rust 中的一种设计模式：
              数据存在 immutable reference，但是允许更改数据；
          interior mutability 绕过 compile 时期的 borrow rule 检查，
          而是在 runtim 时期进行 borrow rules 检查，
          所以我们在使用 interior mutability 的时候，
          需要保证代码在 runtime 时期可以通过 borrow rule 检查，否则将会 panic；
        */

        /*
          Rc<T> 可以拥有多个 owner，Box<T> 和 RefCell<T> 只能拥有一个 owner；
          RefCell<T> 和 Box<T> 以及 reference 的区别在于：
              Box<T> 和 reference 在 compile 时期检查 borrow rule，
              RefCell<T> 在 runtime 时期检查 borrow rule；
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
