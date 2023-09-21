//#[allow(dead_code)]
fn private_function4() {
    println!("called `my4::nested4::private_function4()`");
}
pub fn function4() {
    print!("called `my4::nested4::function4()`, that\n> ");

    private_function4();
}

