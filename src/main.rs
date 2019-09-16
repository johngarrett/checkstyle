use std::fs;
use std::str;
use std::env;
use std::process::Command;

fn main() {
    let filename = env::args().nth(1)
        .expect("Filename is required");

    println!("was passed {}", filename);

    let file_contents = fs::read_to_string(&filename)
        .expect("An error occured reading the file");

    println!("We read:\n{}", file_contents);

    checkstyle(filename);
}

fn checkstyle(filename: String) {
    let checkstyle_script = Command::new("sh")
                            .arg("../src/checkstyle")
                            .arg(&filename)
                            .output()
                            .expect("Could not run the checkstyle script");

    let checkstyle_output = str::from_utf8(&checkstyle_script.stdout)
                                .expect("Error getting checkstyle output");
                            
    println!("{}", checkstyle_output);
}
