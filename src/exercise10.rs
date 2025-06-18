use std::{fs,io::{Read,self}};
use std::io::stdin;
use std::process;
use std::fs::{File};


fn main() {
    
    match write_to_file() {
        Ok(val) => {
            println!("Successfully written to file {val}");
        }
        Err(error) => {
            eprintln!("There was an Error Writing to file {error:?}");
        }
    }

}

fn write_to_file()->Result<String,io::Error> {
    println!("What file would you like to write to?");
    let mut buff = String::new();
    stdin().read_line(&mut buff)?; 
 
    println!("What would you like to write to the file?");
    let mut buff2 = String::new();
    
    stdin().read_line(&mut buff2)?;

    fs::write(buff.trim(), buff2.trim())?;

    Ok(buff) 

}