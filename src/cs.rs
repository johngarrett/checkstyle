use std::process::Command;
mod error;

pub fn run_script(filename: &str) -> String {
    let checkstyle_script = Command::new("sh")
                            .arg("../src/checkstyle")
                            .arg(&filename)
                            .output()
                            .expect("Could not run the checkstyle script");

    return String::from_utf8(checkstyle_script.stdout)
                .expect("Error getting checkstyle output");
}

pub fn handle_errors(script_result: &str, file_contents: String) {
    /*
     * Example script output:
     * 
     * Checkstyle:
     *
     * Javadocs:
     * ~/filename 7: no @author
     */

    let javadoc_start = script_result
                        .find("Javadoc:")
                        .expect("No javadocs output found, did the script run correctly?");
    format_errors(script_result[..javadoc_start].to_string(), &file_contents);
    javadoc_errors(script_result[javadoc_start..].to_string(), &file_contents);
}

pub fn format_errors(std_errors: String, file_contents: &String) {
    for error in std_errors.trim().lines().skip(1) {
        if let Some(colon_index) = error.find(":") {
            match_error(&error[colon_index + 1..]);
        }
            //println!("{}", &error.to_string()[error.find(":").unwrap()+1..]);
    }
}

pub fn javadoc_errors(errors: String, file_contents: &String) {
    println!("JAVADOC {}", errors);
}

fn match_error(error_message: &str) {
    let mut line_number: usize = 0;
    if let Some(index) = error_message.find(":") {
       line_number = error_message[..index].parse::<usize>().unwrap(); 
       println!("{}", line_number);
    }
    let error = &error_message[line_number - 1..];
    println!("{}", error);
}
