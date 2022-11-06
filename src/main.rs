mod cacher;
mod closure;
use closure::generate_workout;

fn main() {
    let intensity = 5;
    let random_number = 0;

    generate_workout(intensity, random_number);
}
