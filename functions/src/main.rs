fn main() {
    another_function(5);
    print_labeled_measurement(5, 'h');
    let x = five();
    let y = plus_one(5);

    println!("The value of x is: {x}");
    println!("The value of y is: {y}");
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// ```
fn five() -> i32 {
    //statement, not an expression
   let x = 5; 
   //expression
   x
}

fn plus_one(y: i32) -> i32 {
    y + 1
}
