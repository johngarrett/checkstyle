use std::fs;
use std::env;
mod cs;

fn main() {
    let filename = env::args().nth(1)
        .expect("Filename is required");

    let file_contents = fs::read_to_string(&filename)
        .expect("An error occured reading the file");
    
    let cs_stdout = cs::run_script(&filename);
    cs::handle_errors(&cs_stdout, file_contents);
}
