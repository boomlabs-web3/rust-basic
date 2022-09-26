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
        let employee = Employee { name, id };

        // println!("{}", name); // error[E0382]: borrow of moved value: `name`
        return employee;
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn id(&self) -> u64 {
        self.id
    }
}

// &Employee is borrowed here
fn borrow_thing(employee: &Employee) {
    println!("borrowed: {}", employee.name);
} // borrowed &Employee is dropped here

fn main() -> Result<(), Box<dyn Error>> {
    let employee = Employee::new_from_default();
    let employee2 = Employee::new("John".to_string(), 101);

    borrow_thing(&employee);

    println!("{} {}", employee.name(), employee.id());
    println!("{} {}", employee2.name, employee2.id);

    Ok(())
}
