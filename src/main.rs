struct Employee {
    name: String,
    id: u64,
}

impl Employee {
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

trait PrintInfo {
    fn print_info(&self);
}

impl PrintInfo for Employee {
    fn print_info(&self) {
        println!(
            "employee's name: {}\nemployee's id: {}\n",
            self.name(),
            self.id()
        );
    }
}

fn main() {
    let employee = Employee::new("Jane".to_string(), 100);

    employee.print_info();
}
