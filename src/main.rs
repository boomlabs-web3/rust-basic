use std::error::Error;

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
}

fn main() -> Result<(), Box<dyn Error>> {
    let employee = Employee::new_from_default();
    let employee2 = Employee::new("John".to_string(), 101);

    println!("{} {}", employee.name, employee.id);
    println!("{} {}", employee2.name, employee2.id);

    Ok(())
}
