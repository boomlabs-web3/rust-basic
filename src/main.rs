mod employee;
use employee::Employee;

mod ided;
use ided::Ided;

mod student;
use student::Student;

// Implement Ided
fn use_ided1(x: &impl Ided) {
    println!("id: {}", x.my_id());
}

// Generic type T must implement Ided trait
fn use_ided2<T: Ided>(x: &T) {
    println!("id: {}", x.my_id());
}

// Box<T>: force type to live in Heap
// dyn: calls to methods on Trait are dynamically dispatched.
fn use_ided3(x: Box<dyn Ided>) {
    println!("id: {}", x.my_id());
}

// #[derive(Debug)]
// struct MiniVec<T, U> {
//     contents: Vec<T>,
//     id: Option<U>,
// }

// impl<T, U> MiniVec<T, U> {
//     fn new() -> Self {
//         Self {
//             contents: Vec::new(),
//             id: None,
//         }
//     }

//     fn push(&mut self, item: T) {
//         self.contents.push(item);
//     }

//     fn set_id(&mut self, id: U) {
//         self.id = Some(id);
//     }
// }

// fn use_minivec<'a, T, U>(v: &'a MiniVec<T, U>) -> &'a [T] {
//     &v.contents
// }

fn main() {
    // let mut v = MiniVec::new();
    // v.push(10);
    // v.set_id(1001);

    // let mut v2 = MiniVec::new();
    // v2.push('c');
    // v2.set_id('a');

    // let mut v3: MiniVec<char, u64> = MiniVec::new();

    // let mut v4 = MiniVec::<Employee, u64>::new();
    // v4.push(Employee::new("John".to_string(), 1));

    // println!(
    //     "v = {:?}\n v2 = {:?}\n v3 = {:?}\n v4 = {:?}",
    //     use_minivec(&v),
    //     use_minivec(&v2),
    //     use_minivec(&v3),
    //     use_minivec(&v4)
    // );
    // v4.set_id(1001);
    // println!("v4 = {:?}", v4);

    // let mut slice = use_minivec(&v2);
    // if let Some(x) = slice.get(0) {
    //     // *x = 'd'; // error[E0594]: cannot assign to `*x`, which is behind a `&` reference
    //     println!("slice = {:?}", x);
    // }

    let employee = Employee::new("Joe".to_string(), 1001);
    use_ided1(&employee);
    use_ided2(&employee);
    use_ided3(Box::new(employee.clone()));

    let student = Student { student_id: 1002 };

    let mut v: Vec<Box<dyn Ided>> = Vec::new();
    v.push(Box::new(employee));
    v.push(Box::new(student));

    for item in v {
        println!("id: {}", item.my_id());
    }
}
