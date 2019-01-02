/*
  Patterns and Matching
      All the Places Patterns Can Be Used
          while let Conditional Loops
*/

pub fn fn_18_01_01_03() {
    println!("-------------------------------------------------- 01");
    {
        let mut stack: Vec<i32> = Vec::new();

        stack.push(1);
        stack.push(2);
        stack.push(3);

        while let Some(x) = stack.pop() {
            println!("{}", x);
        }
    }
}
