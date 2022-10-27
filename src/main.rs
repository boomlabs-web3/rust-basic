#[derive(Clone)]
struct Employee {
    name: String,
    id: u64,
}

impl Employee {
    fn new(name: String, id: u64) -> Employee {
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

#[derive(Clone)]
struct Product {
    name: String,
    price: u64,
    quantity: u64,
    production_date: u64,
}

impl Product {
    fn new(name: String, price: u64, quantity: u64, production_date: u64) -> Product {
        Product {
            name,
            price,
            quantity,
            production_date,
        }
    }

    fn date(&self) -> String {
        let mut date = self.production_date;
        let year: u64 = date / 10000;
        date = date - year * 10000;
        let month = date / 100;
        date = date - month * 100;
        format!("{}/{}/{}", year, month, date)
    }
}

trait PrintInfo {
    fn print_info(&self);
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

impl PrintInfo for Product {
    fn print_info(&self) {
        println!(
            "Product's name: {}\nProduct's price: {}\nProduct quantity: {}\nProduction date: {}",
            self.name,
            self.price,
            self.quantity,
            self.date()
        );
    }
}

fn static_print(data: impl PrintInfo) {
    data.print_info();
}

fn dynamic_print(data: Box<dyn PrintInfo>) {
    data.print_info();
}

fn main() {
    let employee = Employee::new("Jane".to_string(), 100);
    let product = Product::new("Apple".to_string(), 1, 100, 20220924);
    static_print(employee.clone());
    static_print(product.clone());
    dynamic_print(Box::new(employee));
    dynamic_print(Box::new(product));
}
