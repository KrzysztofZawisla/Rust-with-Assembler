extern crate cc;

fn main() {
  cc::Build::new().file("asm/example_number.asm").compile("example_number");
}