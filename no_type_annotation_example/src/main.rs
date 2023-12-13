fn main() {
    //Typescript copium intensifies
    //Diagnostics:
    //1. type annotations needed [E0282]
    //2. consider giving `guess` an explicit type: `: /* Type */` [E0282]
    let guess = "42".parse().expect("Not a number!");
    //Type me harder daddy
    let typed_guess: u32 = "42".parse().expect("Not a number!");
}
