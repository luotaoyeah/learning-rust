/*
  Patterns and Matching
      Refutability: Whether a Pattern Might Fail to Match
*/

pub fn fn_18_02_01_01() {
    println!("-------------------------------------------------- 01");
    {
        /*
           irrefutable pattern：对于任意值，该 pattern 都能匹配；
               如：
                   let 语句
                   for 循环
                   function 参数
           refutable pattern：对于某些值，该 pattern 不能匹配；
               如：
                   if let
                   while let
                   match arm
        */

        /*
          let 中的 pattern 需要是一个 irrefutable pattern，
          而 Some(x) 是一个 refutable pattern；
        */
        let Some(x) = Some(9); // [E0005]: refutable pattern in local binding: `None` not covered

        /*
          if let 中的 pattern 需要是一个 refutable pattern，
          而 x 是一个 irrefutable pattern；
        */
        if let x = Some(9) {} // [E0162]: irrefutable if-let pattern
    }
}
