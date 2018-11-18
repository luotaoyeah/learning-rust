/*
  Advanced Features
      Advanced Traits
          Fully Qualified Syntax for Disambiguation: Calling Methods with the Same Name
*/

pub fn fn_19_03_03_01() {
    println!("-------------------------------------------------- 01");
    {
        /*
          当某个类型实现的多个 trait 具有同名方法时，
          如何指定具体调用的时哪个方法？
        */

        trait Pilot {
            fn fly(&self);
        }

        trait Wizard {
            fn fly(&self);
        }

        struct Human;

        impl Human {
            fn fly(&self) {
                println!("HUMAN FLY");
            }
        }

        impl Pilot for Human {
            fn fly(&self) {
                println!("PILOT FLY");
            }
        }

        impl Wizard for Human {
            fn fly(&self) {
                println!("WIZARD FLY");
            }
        }

        let human = Human;
        /*
          默认调用的是类型自己定义的方法
        */
        human.fly(); // HUMAN FLY

        /*
          通过下面的方式，分别调用各个 trait 中定义的方法；
        */
        Human::fly(&human);
        Pilot::fly(&human);
        Wizard::fly(&human);
    }

    println!("-------------------------------------------------- 02");
    {
        /*
          如果不是方法，而是 associated function 呢？
        */

        trait Animal {
            fn walk();
        }

        struct Dog;

        impl Dog {
            fn walk() {
                println!("DOG WALK");
            }
        }

        impl Animal for Dog {
            fn walk() {
                println!("ANIMAL WALK");
            }
        }

        Dog::walk(); // DOG WALK

        /*
          使用全限定名（fully qualified syntax）语法调用 associated function
        */
        <Dog as Animal>::walk(); // ANIMAL WALK
    }
}
