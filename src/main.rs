mod employee;
use employee::Employee;

mod ided;

mod student;

#[derive(Debug, Clone)]
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

fn use_bigger<'a, 'b: 'a, T>(v: &'a MiniVec<T, u64>, w: &'b MiniVec<T, u64>) -> &'a [T] {
    if v.id.unwrap() > w.id.unwrap() {
        &v.contents
    } else {
        &w.contents
    }
}

fn main() {
    let mut v = MiniVec::new();
    v.push(10);
    v.set_id(1001);

    let mut v2 = MiniVec::new();
    v2.push('c');
    v2.set_id('a');

    let v3: MiniVec<char, u64> = MiniVec::new();

    let mut v4 = MiniVec::<Employee, u64>::new();
    v4.push(Employee::new("John".to_string(), 5));
    v4.set_id(v4.contents[0].id());

    println!(
        "v = {:?}\n v2 = {:?}\n v3 = {:?}\n v4 = {:?}",
        use_minivec(&v),
        use_minivec(&v2),
        use_minivec(&v3),
        use_minivec(&v4)
    );

    let mut v5 = MiniVec::<Employee, u64>::new();
    v5.push(Employee::new("Joe".to_string(), 10));
    v5.set_id(v5.contents[0].id());

    println!("max(v4, v5) = {:?}", use_bigger(&v4, &v5));
}
