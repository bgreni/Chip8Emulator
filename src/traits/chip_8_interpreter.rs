use crate::interpreter::Interpreter;

pub trait Chip8Interpreter {
    fn load_program(self, filename: &str);
    fn fetch_next_instruction(self);
    fn execute_current_instruction(self);
}

impl Chip8Interpreter for Interpreter {
    fn load_program(self, _filename: &str) {
        // let loader = ProgramLoader::new();
        // let file_contents = loader.load_file_contents(filename);
        // println!("{}", file_contents);
    }

    fn fetch_next_instruction(self) {

    }

    fn execute_current_instruction(self) {

    }
}