mod employee;
use employee::Employee;

mod ided;
use ided::use_ided;

mod student;
use student::Student;

#[derive(Debug)]
struct MiniVec<T> {
    contents: Vec<T>,
}

impl<T> MiniVec<T> {
    fn new() -> Self {
        Self {
            contents: Vec::new(),
        }
    }

    fn push(&mut self, item: T) {
        self.contents.push(item);
    }
}

fn main() {
    let mut v = MiniVec::new();
    v.push(10);

    let mut v2 = MiniVec::new();
    v2.push('c');

    let mut v3: MiniVec<char> = MiniVec::new();

    let mut v4 = MiniVec::<Employee>::new();
    v4.push(Employee::new("John".to_string(), 1));

    println!(
        "v = {:?}\n v2 = {:?}\n v3 = {:?}\n v4 = {:?}",
        v, v2, v3, v4
    );
}
