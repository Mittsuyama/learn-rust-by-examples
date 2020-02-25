#[allow(dead_code)]

fn function() {
    println!("The outest!");
}

mod my_mod {
    pub fn function() {
        println!("Outer mod!");
    }
    pub mod innter_mod {
        pub fn function() {
            println!("Inner mod!");
        }
        // 使用 self 和 super 访问自身、父模块，消除歧义，减少路径硬编码
        pub fn call_function() {
            center_mod::function();
            self::function();
            super::function();
            super::another_mod::function();
        }
        pub mod center_mod {
            pub fn function() {
                println!("Center mod!");
            }
        }
    }
    pub mod another_mod  {
        pub fn function() {
            println!("Another mod!");
        }
    }
}

fn main() {
    function();
    my_mod::innter_mod::call_function();
}