use crate::printinfo::PrintInfo;

#[derive(Clone)]
pub struct Employee {
    name: String,
    id: u64,
}

impl Employee {
    pub fn new(name: String, id: u64) -> Employee {
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

impl PrintInfo for Employee {
    fn print_info(&self) {
        println!(
            "employee's name: {}\nemployee's id: {}\n",
            self.name(),
            self.id()
        );
    }
}
