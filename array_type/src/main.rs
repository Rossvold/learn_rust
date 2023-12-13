use std::io;
fn main() {

    //Array types are very strict, notice the type of _a is actually set to a 
    //lenght of 5 without having to specify it.
    let a = [1, 2, 3, 4, 5];

    //This is really nice if you are working on something that's always fixed,
    //like time.
    let _months: [&str; 12] = ["January", "February", "March", "April", "May", "June", "July",
    "August", "September", "October", "November", "December"];

    //You can also to contain the same value for each element by specifying the
    //initial value, followed by a semicolon, and then the length of the array
    //in square brackets, as shown here:
    let _b = [3; 5];
    //This is the same as writing let a = [3, 3, 3, 3, 3];



    //Accessing array elements
    let first = a[0];
    println!("The value of first is: {first}");
    let second = a[1];
    println!("The value of second is: {second}");



    //Example of an invalid array access
    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");

}
