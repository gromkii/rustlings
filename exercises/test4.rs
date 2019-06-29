// test4.rs
// This test covers the sections:
// - Modules
// - Macros

// Write a macro that passes the test! No hints this time, you can do it!

macro_rules! my_macro {
    () => {
        println!("Shit")
    };
    ($val: expr) => {
        if $val == "world!" {
            String::from("Hello world!")
        }
    }
}

fn main() {
    if my_macro!("world!") != "Hello world!" {
        panic!("Oh no! Wrong output!");
    }
}
