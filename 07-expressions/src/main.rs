
fn expressions_2() {
    println!("");
    println!("start expressions_2");

    let x = 5u32;

    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        // This expression will be assigned to `y`
        x_cube + x_squared + x
    };

    let z = {
        // The semicolon suppresses this expression and `()` is assigned to `z`
        //2 * x; // the arithmetic operation produces a value
        let _ = 2 * x;
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);

    println!("end expressions_2");
    println!("");
}

#[allow(path_statements)] //`#[warn(path_statements)]` on by default
fn expressions_1() {
    println!("");
    println!("start expressions_1");

    // variable binding
    let x = 5;

    // expression;
    x;
    //x + 1; //the arithmetic operation produces a value
    15;

    println!("end expressions_1");
    println!("");
}

fn main() {
    expressions_1();
    expressions_2();
}
