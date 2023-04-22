// functions5.rs
// Execute `rustlings hint functions5` or use the `hint` watch subcommand for a hint.


// a statement always returns ()
// so num * num can’t have a ; for it to return a number

fn main() {
    let answer = square(3);
    println!("The square of 3 is {}", answer);
}

fn square(num: i32) -> i32 {
    num * num
}
