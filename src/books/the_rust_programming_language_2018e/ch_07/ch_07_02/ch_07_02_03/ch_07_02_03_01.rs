/*
   Modules
       Privacy Rules
 */

pub fn fn_07_02_03_01() {
    println!("-------------------------------------------------- 01");
    {
        /*
           访问规则：
               1. public：可以被所有父模块访问；
               2. private：可以被直接父模块（immediate parent module）以及该父模块的子模块（child module）访问；
         */

        mod outermost {
            pub fn middle_function() {}
            fn middle_private_function() {}
            mod inside {
                pub fn inner_function() {}
                fn inner_private_function() {}
            }
        }

        fn try_me() {
            /*
               outermost 是 private 的，可以被 mimediate parent module（root module）及其 child module 访问；
             */
            outermost::middle_function();
            outermost::middle_private_function();
            outermost::inside::inner_function();
            outermost::inside::inner_private_function();
        }
    }
}
