mod employee;
use employee::Employee;

mod ided;
use ided::use_ided;

mod student;
use student::Student;

#[derive(Debug)]
struct MiniVec {
    contents: Vec<u64>,
}

impl MiniVec {
    fn new() -> Self {
        Self {
            contents: Vec::new(),
        }
    }

    fn push(&mut self, item: u64) {
        self.contents.push(item);
    }
}

fn main() {
    let mut v = MiniVec::new();
    v.push(10);

    println!("v = {:?}", v);
}
