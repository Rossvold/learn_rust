fn main() {
    let my_string = String::from("hello_world");

    let word = first_word(&my_string);
    println!("from fn first_word: {}", word);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word(&my_string_literal[0..3]);
    println!("0..3: {}", word);
    let word = first_word(&my_string_literal[..]);
    println!("just '..': {}", word);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
    println!("literal_word: {}", word);

}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
