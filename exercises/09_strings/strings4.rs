// strings4.rs
//
// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!
//
// No hints this time!

fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    string("blue".to_string());
    string("red".to_string());
    string_slice(&String::from("hi"));
    string("rust is fun!".to_owned());
    string("nice weather".into());
    string_slice(format!("Interpolation {}", "Station".to_string()).as_str());
    string_slice(&String::from("abc".to_string())[0..1]);
    string("  hello there ".trim().to_string());
    string_slice(&"Happy Monday!".to_string().replace("Mon", "Tues"));
    string("mY sHiFt KeY iS sTiCkY".to_lowercase().to_string());
}
