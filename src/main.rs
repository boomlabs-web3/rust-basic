fn greet(x: &str) -> String {
    format!("{}", x)
}

fn main() {
    let my_greeting = "Hello, world!";
    let result = greet(my_greeting);
    println!("{}", result);
}
