use crate::printinfo::PrintInfo;

#[derive(Clone)]
pub struct Product {
    name: String,
    price: u64,
    quantity: u64,
    production_date: u64,
}

impl Product {
    pub fn new(name: String, price: u64, quantity: u64, production_date: u64) -> Product {
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
