// TODO: Fix the compiler error by adding one or two characters.
#[rustfmt::skip]
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    }; // ; must be added
    ($val:expr) => {
        println!("Look at this other macro: {}", $val);
    }; // ; must be added
}

fn main() {
    my_macro!();
    my_macro!(7777);
}
