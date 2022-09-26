#[derive(Debug)]
struct Employee {
    name: String,
    id: u64,
}

fn main() {
    let employee = Employee {
        name: "John".to_string(),
        id: 101,
    };
    println!("{:#?}", employee);
}
