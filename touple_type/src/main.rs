fn main() {
    //Touples are tough as nails in it's consistency.
    //Typescript soyboys are crying in the corner.(me included)
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    //Notice that each element in the touple is assigned the correct type?
    let (_x, y, _z) = tup;

    println!("The value of y is: {y}");

    index_touple();
}
fn index_touple() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;
    println!("The value of five_hundred is: {five_hundred}");

    let six_point_four = x.1;
    println!("The value of six_point_four is: {six_point_four}");

    let one = x.2;
    println!("The value of one is: {one}");
}
