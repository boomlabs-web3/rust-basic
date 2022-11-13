#[repr(C)]
union MyUnion {
    f1: u32,
    f2: u64,
}

fn main() {
    let u = MyUnion { f1: 1 };
    println!("{}, {}", unsafe { u.f1 }, unsafe { u.f2 });
}
