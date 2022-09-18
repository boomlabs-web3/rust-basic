mod employee;
use employee::Employee;

mod ided;
use ided::use_ided;

mod student;
use student::Student;

#[derive(Debug)]
struct MiniVec<T, U> {
    contents: Vec<T>,
    id: Option<U>,
}

impl<T, U> MiniVec<T, U> {
    fn new() -> Self {
        Self {
            contents: Vec::new(),
            id: None,
        }
    }

    fn push(&mut self, item: T) {
        self.contents.push(item);
    }

    fn set_id(&mut self, id: U) {
        self.id = Some(id);
    }
}

fn main() {
    let mut v = MiniVec::new();
    v.push(10);
    v.set_id(1001);

    let mut v2 = MiniVec::new();
    v2.push('c');
    v2.set_id('a');

    let mut v3: MiniVec<char, u64> = MiniVec::new();

    let mut v4 = MiniVec::<Employee, u64>::new();
    v4.push(Employee::new("John".to_string(), 1));

    println!(
        "v = {:?}\n v2 = {:?}\n v3 = {:?}\n v4 = {:?}",
        v, v2, v3, v4
    );
}
