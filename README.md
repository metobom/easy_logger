# easy_logger
easy_logger is simple logger for Rust programming language that I wrote for getting used to Rust while I follow Rust book that is available for free in here: https://doc.rust-lang.org/book/ 
* I would really appreciate any criticism or advice.

# Including in your project
* add 'easy_logger = { path = "/path/to/easy_logger" }' to Cargo.toml under [dependencies].
* Then just build your project.

# Usage

* Sample usage: 
```
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


```
* Output format -> DATE | DEBUG_TYPE | THREAD_NAME:LOG_LINE_NUMBER - LOG_TEXT

* Sample output:
```
2022-03-30T01:48:26.792058940+03:00 | INFO | Some("main"):24 - Jumping ahead
2022-03-30T01:48:26.792192985+03:00 | DEBUG | Some("main"):25 - Try finger but hole
2022-03-30T01:48:26.792370707+03:00 | ERROR | Some("main"):26 - Praise the sun !!!
```
