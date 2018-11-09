/*
  More About Cargo and Crates.io
      Customizing Builds with Release Profiles
*/

pub fn fn_14_01_01_01() {
    println!("-------------------------------------------------- 01");
    {
        /*
          cargo 默认有两个 profile：dev 和 release，
          当执行 cargo build 时，使用 dev，
          当执行 cargo build --release 时，使用 release；
        
          可以在 Cargo.toml 中，通过 [profile.dev] 和 [profile.release] 设置各个选项；
        */
    }
}
