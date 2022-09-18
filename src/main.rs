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

fn use_minivec<'a, T, U>(v: &'a MiniVec<T, U>) -> &'a [T] {
    &v.contents
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
        use_minivec(&v),
        use_minivec(&v2),
        use_minivec(&v3),
        use_minivec(&v4)
    );
    v4.set_id(1001);
    println!("v4 = {:?}", v4);

    let mut slice = use_minivec(&v2);
    if let Some(x) = slice.get(0) {
        // *x = 'd'; // error[E0594]: cannot assign to `*x`, which is behind a `&` reference
        println!("slice = {:?}", x);
    }
}
