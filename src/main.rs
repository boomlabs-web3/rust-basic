fn greet(x: String) {
    println!("Hello to {}", x);
}

fn main() {
    let my_greeting = "Hello, world!".to_string();
    greet(my_greeting);
}
