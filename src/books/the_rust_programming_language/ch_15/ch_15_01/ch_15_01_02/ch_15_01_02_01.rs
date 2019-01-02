/*
  Smart Pointers
      Using Box<T> to Point to Data on the Heap
          Enabling Recursive Types with Boxes
              Using Box<T> to Get a Recursive Type with a Known Size
*/

pub fn fn_15_01_02_01() {
    println!("-------------------------------------------------- 01");
    {
        /*
          在 compile 时期，rust 需要知道某个类型占据的内存大小，
          但是有些类型在 compile 时期是无法知道占据的内存大小的，
          比如 recursive type；
        */

        /*
                enum List {
                    Cons(i32, List),
                    Nil,
                }
        
                // [E0072]: recursive type has infinite size
                let list = List::Cons(1, List::Cons(2, List::Cons(3, List::Nil)));
        */
    }

    println!("-------------------------------------------------- 02");
    {
        enum List {
            Cons(i32, Box<List>),
            Nil,
        }

        let list = List::Cons(
            1,
            Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
        );
    }
}
