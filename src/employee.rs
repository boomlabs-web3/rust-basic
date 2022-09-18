use crate::ided::Ided;

#[derive(Debug)]
pub struct EmployeeRecords {
    idx: usize,
    employees: Vec<Employee>,
}

impl EmployeeRecords {
    pub fn new() -> Self {
        Self {
            idx: 0,
            employees: Vec::new(),
        }
    }

    pub fn push(&mut self, employee: Employee) {
        self.employees.push(employee);
    }

    pub fn get(&self, id: u64) -> Option<&Employee> {
        self.employees.iter().find(|e| e.id == id)
    }
}

impl Iterator for EmployeeRecords {
    type Item = Employee;

    // next() is the only required method
    fn next(&mut self) -> Option<Self::Item> {
        if self.employees.len() > self.idx {
            let output = self.employees[self.idx].clone();
            self.idx += 1;
            return Some(output);
        } else {
            return None;
        }
    }
}

#[derive(Debug, Clone)]
pub struct Employee {
    name: String,
    id: u64,
}

impl Employee {
    pub fn new(name: String, id: u64) -> Employee {
        Employee { name, id }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    fn update(&mut self, name: String, id: u64) {
        self.name = name;
        self.id = id;
    }

    pub fn id(&self) -> u64 {
        self.id
    }
}

impl Ided for Employee {
    fn my_id(&self) -> u64 {
        self.id
    }
}
