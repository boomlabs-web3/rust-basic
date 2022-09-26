use parking_lot::Mutex;
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

    // expanded or un-elided lifetime.
    fn name<'a>(&'a self) -> &'a str {
        &self.name
    }

    fn id(&self) -> u64 {
        self.id
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let employee = Employee::new_from_default();
    let employee2 = Employee::new("John".to_string(), 101);

    let locked_employee = Mutex::new(employee);

    let unlocked_employee = locked_employee.lock();

    println!("{} {}", unlocked_employee.name(), unlocked_employee.id());

    Ok(())
}
