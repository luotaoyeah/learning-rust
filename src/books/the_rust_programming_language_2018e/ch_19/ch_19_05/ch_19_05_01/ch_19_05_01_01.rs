/*
  Advanced Features
      Advanced Functions and Closures
          Function Pointers
*/

pub fn fn_19_05_01_01() {
    println!("-------------------------------------------------- 01");
    {
        /*
          closure 可以被作为参数传递，
          function pointer 也可以被作为参数传递；
        
          closure 使用 Fn 特性来标识类型，
          function pointer 使用 fn 类型来标识类型；
        */

        fn add_one(x: i32) -> i32 {
            x + 1
        }

        fn add_twice(f: fn(i32) -> i32, x: i32) -> i32 {
            f(x) + f(x)
        }

        println!("{}", add_twice(add_one, 5)); // 12
    }

    println!("-------------------------------------------------- 02");
    {
        /*
          function pointer 实现了 closure 的三个 trait：Fn，FnMut，FnOnce，
          因此期望 closure 的地方，也可以传递一个 function pointer；
        */

        /* 使用 closure 作为 map() 的参数 */
        println!(
            "{:?}",
            vec![1, 2, 3]
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
        ); // ["1", "2", "3"]

        /* 使用 function pointer 作为 map() 的参数 */
        println!(
            "{:?}",
            vec![1, 2, 3]
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<String>>()
        ); // ["1", "2", "3"]
    }
}
