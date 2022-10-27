pub trait PrintInfo {
    fn print_info(&self);
}

pub fn static_print(data: impl PrintInfo) {
    data.print_info();
}

pub fn dynamic_print(data: Box<dyn PrintInfo>) {
    data.print_info();
}
