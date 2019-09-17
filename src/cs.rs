use std::process::Command;

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
     * 0
     *
     * Javadocs:
     * ~/filename 7: no @author
     * 1
     */

    let javadoc_start = script_result.find("Javadoc:").unwrap();  //the script should always contain this string
    format_errors(script_result[..javadoc_start].to_string(), &file_contents);
    javadoc_errors(script_result[javadoc_start..].to_string(), &file_contents);
}
pub fn format_errors(errors: String, file_contents: &String) {
    println!("FORMAT {}", errors);
}

pub fn javadoc_errors(errors: String, file_contents: &String) {
    println!("JAVADOC {}", errors);
}
