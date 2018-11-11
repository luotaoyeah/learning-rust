/*
  Fearless Concurrency
      Extensible Concurrency with the Sync and Send Traits
          Allowing Transference of Ownership Between Threads with Send
          Allowing Access from Multiple Threads with Sync
          Implementing Send and Sync Manually Is Unsafe
*/

pub fn fn_16_04_01_01() {
    println!("-------------------------------------------------- 01");
    {
        /*
            实现了 std::marker::Send 特性的类型，可以在多个线程之间转移 ownership，
            实现了 std::marker::Sync 特性的特性，可以在多个线程之间转移 reference，
                即如果类型 T 是一个 Sync，则 &T 是一个 Send；
            rust 中大部分类型都实现了 Send 特性和 Sync 特性；
        */
    }
}
