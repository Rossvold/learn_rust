fn main() {
    // sum becomes an i32 type
    // summation
    let _sum = 5 + 10;
    println!("The value of _sum is: {_sum}");

    // subtraction
    // difference becomes an f64 type
    let _difference = 95.5 - 4.3;
    println!("The value of _difference is: {_difference}");

    // multiplication
    // product becomes an i32 type
    let _product = 4 * 30;
    println!("The value of _product is: {_product}");

    // division
    // quotient becomes an f64 type
    let _quotient = 56.7 / 32.2;
    println!("The value of _quotient is: {_quotient}");
    //truncated becomes an i32 type
    //here using a Unsigned type would cause a overflow, and the results would
    //be 254
    let _truncated = -5 / 3; // Results in -1
    println!("The value of _truncated is: {_truncated}");

    // remainder
    let _remainder = 43 % 5;
    println!("The value of _remainder is: {_remainder}");
}
