// use std::fs::File;
// use std::io::Read;
// use std::path::Path;

// pub struct ProgramLoader {
//     file_len: u16
// }

// impl ProgramLoader {
    // pub fn load_file_contents(self, filename: &str) -> String {
    //     let file_path = format!("{}/programs/{}", env!("CARGO_MANIFEST_DIR"), filename);

    //     println!("{}", file_path);

    //     let f = File::open(file_path);

    //     let mut f = match f {
    //         Ok(file) => file,
    //         Err(error) => panic!("Could not open program file: {:?}", error)
    //     };
    //     let mut file_string = String::new();
    //     match f.read_to_string(&mut file_string) {
    //         Ok(res) => res,
    //         Err(error) => panic!("Reading file failed: {:?}", error)
    //     };
    //     return file_string;
    // }

    // pub fn split_file_lines(contents: &str) -> Vec<&str> {
    //     return contents.lines().collect();
    // }

    // pub fn new() -> ProgramLoader {
    //     return ProgramLoader {file_len: 0};
    // }
// }