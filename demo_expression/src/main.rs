// ref: https://course.rs/basic/base-type/statement-expression.html#%E8%A1%A8%E8%BE%BE%E5%BC%8F
fn main() {
    // the block wrapped by the curly braces is an expression
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
}
