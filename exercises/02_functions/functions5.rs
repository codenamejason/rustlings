// functions5.rs
//
// Execute `rustlings hint functions5` or use the `hint` watch subcommand for a
// hint.

fn main() {
    let number = 10;
    let answer = square(10);
    println!("The square of {} is {}", number, answer);
}

fn square(num: i32) -> i32 {
    return num * num;
}
