#[derive(Debug)]
struct Employee {
    name: String,
    id: u64,
}

enum IsTrue {
    True(u64),
    False,
}

fn main() {
    let my_value = IsTrue::True(100);

    match my_value {
        IsTrue::True(x) => println!("True {}", x),
        IsTrue::False => println!("False"),
    }
}
