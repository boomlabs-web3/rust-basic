mod employee;
use employee::Employee;

mod printinfo;
use printinfo::{dynamic_print, static_print};

mod product;
use product::Product;

fn main() {
    let employee = Employee::new("Jane".to_string(), 100);
    let product = Product::new("Apple".to_string(), 1, 100, 20220924);
    static_print(employee.clone());
    static_print(product.clone());
    dynamic_print(Box::new(employee));
    dynamic_print(Box::new(product));
}
