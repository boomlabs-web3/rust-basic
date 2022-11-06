mod employee;
use employee::{Employee, EmployeeRecords};

mod printinfo;

fn main() {
    let john = Employee::new(String::from("John"), 100);
    let jane = Employee::new(String::from("Jane"), 101);
    let tom = Employee::new(String::from("Tom"), 102);

    let mut employee_records = EmployeeRecords::new();
    employee_records.push(john);
    EmployeeRecords::push(&mut employee_records, jane);
    employee_records.push(tom);

    println!("{:#?}", employee_records);
    println!("Get 101: {:?}", employee_records.get(101));
    println!("Get 100: {:?}", employee_records.get(100));
    println!("---------------------------------------------\n");

    println!("{:#?}", employee_records);
    println!("Next: {:?}", employee_records.next());
    println!("---------------------------------------------\n");

    println!("{:#?}", employee_records);
    println!("Get 100: {:?}", employee_records.get(100));
    println!("---------------------------------------------\n");

    for employee in &mut employee_records {
        println!("For {:#?}", employee);
    }
    println!("{:#?}", employee_records);
}
