use std::{
    fs::File, 
    io::{Read}, 
    path::Path
};
use crate::corpus::Inputs;


#[derive(Debug)]
pub struct BytesInput {
    pub bytes: Vec<u8>,
}

impl Inputs for BytesInput {

    fn read_file<P>(path: P) -> Self 
    where
        P: AsRef<Path>,
    {
        let mut file = File::open(path).unwrap();
        let mut bytes: Vec<u8> = vec![];
        file.read_to_end(&mut bytes).unwrap();
        BytesInput::new(bytes)
    }
}

impl BytesInput {
    pub fn new(bytes: Vec<u8>) -> Self {
        Self {bytes}
    }
}