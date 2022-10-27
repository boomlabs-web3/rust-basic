struct Employee {
    name: String,
    id: u64,
}

impl Employee {
    fn new_from_default() -> Employee {
        Employee {
            name: "default".to_string(),
            id: 100,
        }
    }

    fn new(name: String, id: u64) -> Employee {
        Employee { name, id }
    }

    // expanded or un-elided lifetime.
    fn name(&self) -> &str {
        &self.name
    }

    fn id(&self) -> u64 {
        self.id
    }
}

fn main() {
    let employee = Employee::new_from_default();

    println!("{} {}", employee.name(), employee.id());
}
