#[link(name = "example_number")]
extern "C" {
    fn example_number() -> u16;
}

fn main() {
    let num = unsafe { example_number() };
    println!("{}", num);
}
