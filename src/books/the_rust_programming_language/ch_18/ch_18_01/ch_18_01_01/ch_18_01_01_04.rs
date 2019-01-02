/*
  Patterns and Matching
      All the Places Patterns Can Be Used
          for Loops
*/

pub fn fn_18_01_01_04() {
    println!("-------------------------------------------------- 01");
    {
        /*
          在 for 循环中，for 和 in 中间的就是 pattern；
        */

        let vec: Vec<char> = vec!['a', 'b', 'c'];

        for (index, value) in vec.iter().enumerate() {
            println!("{}: {}", index, value);
        }
    }
}
