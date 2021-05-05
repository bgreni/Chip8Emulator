use std::fs::File;
use std::io::Read;
use std::path::Path;

pub struct ProgramHandler {
    file_len: usize,
    file_contents: Vec<u8>
}

impl ProgramHandler {
    pub fn load_file_contents(&mut self, filename: &str) -> Vec<u8> {
        let mut f = File::open(filename).expect("File does not exist");

        f.read_to_end(&mut self.file_contents);

        self.file_len = self.file_contents.len();

        return self.file_contents.clone();
    }

    pub fn new() -> ProgramHandler {
        return ProgramHandler {file_len: 0, file_contents: Vec::new()};
    }
}