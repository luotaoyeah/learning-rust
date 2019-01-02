/*
  Patterns and Matching
      Pattern Syntax
          Ignoring Values in a Pattern
              Ignoring Parts of a Value with a Nested _
*/

pub fn fn_18_03_03_02() {
    println!("-------------------------------------------------- 01");
    {
        /*
          _ 用在其他的 pattern 中，可以匹配某个部分；
        */

        let mut setting_value = Some(5);
        let new_setting_value = Some(10);

        match (setting_value, new_setting_value) {
            (Some(_), Some(_)) => println!("not setting"),
            _ => setting_value = new_setting_value,
        }
    }

    println!("-------------------------------------------------- 02");
    {
        let (a, _, c, _, e) = (1, 2, 3, 4, 5);
        println!("{} {} {}", a, c, e); // 1 3 5
    }
}
