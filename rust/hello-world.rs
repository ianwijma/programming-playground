
fn main () {
    // Assigning a value
    let first = "Ian";

    // The mut keyword indicates you can mutate the variable
    let mut last = "";

    // We print out what we have thus far
    println!("Hello {} {}", first, last);

    // Here we update the last variable
    last = "Wijma";

    // This is how string interpolation works within Rust
    println!("Hello {} {}", first, last);

    // A way to dictate where for where, you can use keywords in the brackets
    println!("Hello {one} {two}", one=last, two=first);
}