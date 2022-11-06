mod function;
use function::generate_workout;

fn main() {
    let intensity = 5;
    let random_number = 3;

    generate_workout(intensity, random_number);
}
