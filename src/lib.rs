
 
use chrono;
use std;
use std::fs::{File}; 
use std::io::prelude::*;

pub struct  Logger{pub log_file: File}  

impl Logger {
    
    #[track_caller]   
    pub fn info(&self, text: &String) {
        let caller_location = std::panic::Location::caller();
        let line_number = caller_location.line();
        println!("{:?} | INFO | {:?}:{} - {}", chrono::offset::Local::now(), std::thread::current().name(), line_number, String::from(text));
        
        writeln!(&self.log_file, "{:?} | INFO | {:?}:{} - {}", chrono::offset::Local::now(), std::thread::current().name(), line_number, text).expect("Couldn't write to file!");
        
    }
 
    #[track_caller]
    pub fn debug(&self, text: &str) {
        let caller_location = std::panic::Location::caller();
        let line_number = caller_location.line(); 
        println!("{:?} | DEBUG | {:?}:{} - {}", chrono::offset::Local::now(), std::thread::current().name(), line_number, String::from(text ));

        writeln!(&self.log_file, "{:?} | DEBUG | {:?}:{} - {}", chrono::offset::Local::now(), std::thread::current().name(), line_number, String::from(text)).expect("Couldn't write to file!");
         
    }

    #[track_caller] 
    pub fn error(&self, text: &str) { 
        let caller_location = std::panic::Location::caller();
        let line_number = caller_location.line();
        println!("{:?} | ERROR | {:?}:{} - {}", chrono::offset::Local::now(), std::thread::current().name(), line_number, String::from(text)); 

        writeln!(&self.log_file, "{:?} | ERROR | {:?}:{} - {}", chrono::offset::Local::now(), std::thread::current().name(), line_number, String::from(text)).expect("Couldn't write to file!");
         
    }  
}