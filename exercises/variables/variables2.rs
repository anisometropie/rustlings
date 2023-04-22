// variables2.rs
// Execute `rustlings hint variables2` or use the `hint` watch subcommand for a hint.

// variable cannot be left unassigned, even with a type annotation
// x has to be an integer to compare with 10

fn main() {
    let x = 3;
    if x == 10 {
        println!("x is ten!");
    } else {
        println!("x is not ten!");
    }
}
