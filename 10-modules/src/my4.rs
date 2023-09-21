// Similarly `mod inaccessible4` and `mod nested4` will locate the `nested4.rs`
// and `inaccessible4.rs` files and insert them here under their respective
// modules
mod inaccessible4;
pub mod nested4;

pub fn function4() {
    print!("called `my4::function4()`, that\n> ");
    inaccessible4::public_function4();
}

fn private_function4() {
    println!("called `my4::private_function4()`");
}

pub fn indirect_access4() {
    print!("called `my4::indirect_access4()`, that\n> ");

    private_function4();
}