use easy_logger::{Logger}; 
use std::fs::{OpenOptions}; 

fn main() {
    // state your log file's name.
    let log_filename = "mmama.log";
    // check if a new file should be created or not.
    let is_create = !std::path::Path::new(log_filename).exists();
    // create file
    let my_log_file = OpenOptions::new()
        .write(true)
        .create_new(is_create)
        .append(true)
        .open(log_filename)
        .expect("Couldn't open file!");
    
    // pass created file to Logger 
    let logger = Logger{log_file: my_log_file}; 
    
    // log as you like. all logger functions expects &String.
    logger.info(&format!("{}", "Jumping ahead"));
    logger.debug(&format!("{}", "Try finger but hole"));
    logger.error(&format!("{} {}", "Praise the sun", "!!!"));
}  