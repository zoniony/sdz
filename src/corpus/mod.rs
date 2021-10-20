
pub mod inputs;
pub use inputs::BytesInput;
pub use std::{
    io::Error,
    path::Path,
};

pub trait Inputs {
    fn read_file<P>(path: P) -> Self where P: AsRef<Path>;
}


