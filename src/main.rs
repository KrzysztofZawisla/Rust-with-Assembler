#[link(name = "example_number")]
extern "C" {
    fn example_number() -> u16;
}

fn main() {
    let sum = unsafe { example_number() };
    println!("{}", sum);
}
