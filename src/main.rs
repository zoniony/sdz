pub mod corpus;
use corpus::{BytesInput, Inputs};

fn main() {
    let input: BytesInput = Inputs::read_file("src/test");
    println!("{:?}",input);
}