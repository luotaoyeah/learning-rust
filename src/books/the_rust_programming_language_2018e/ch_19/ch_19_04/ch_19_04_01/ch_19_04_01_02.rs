/*
  Advanced Features
      Advanced Types
          The Never Type that Never Returns
*/

pub fn fn_19_04_01_01() {
    println!("-------------------------------------------------- 01");
    {
        /*
          ! 是一个特殊的类型，
          当函数没有返回时，可以将返回类型标注为 ！（never）；
        */

        fn fn_01() {
            println!("fn_01()");
        }
        fn_01();

        /*
                fn fn_02() -> ! {
                    panic!("fn_02()")
                }
                fn_02();
        */
    }

    println!("-------------------------------------------------- 02");
    {
        /*
          ! 类型一般用在下面几个地方：
              panic!() 的返回值为 !
        */

        /*
                fn fn_01() -> ! {
                    panic!("PANIC")
                }
        */

        let mut i = 0;
        loop {
            i += 1;

            /*
              当 match 的某个 arm 是 continue 时，
              该 match 的值为 !
            */
            let t = match i {
                i if i > 5 => i,
                _ => continue,
            };

            println!("{}", t);

            if i > 9 {
                break;
            }
        }

        /*
          当 loop 循环是一个死循环时，其值为 !
        */
    }
}
