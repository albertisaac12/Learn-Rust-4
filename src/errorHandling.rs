use std::fs::{self, File};
use std::io::{stdin,Read,self};
use std::process;


fn main() {
    
    // process::exit(0x00); // zero all went well, > 0 something went wrong
    // panic!("message");
    
    // eprintln!("This is an error hahahah");
    // println!("Hello");

    // let file = File::open("abc.txt").expect("File does Not exist");

    // let file = File::open("abc.txt");
    // println!("{file:#?}");
    // let file = File::open("ab.txt");
    // println!("{file:#?}");
    
    let file_result = read_file();

    match file_result {
        Ok(val) => {
            println!("The contents of the file are : {val}");
        }
        Err(error) => {
            eprintln!("There was an error {:#?}",error);
            // process::exit(1); // no need because nothing else is present after this
        }
    }



}

/* use std::process; */
// Essentially the exit function in process module makes sure that the code terminates
// eprintln!(); // prints the message to standard error stream instead of the standard output stream.

fn read_file() -> Result<String, io::Error>{
    println!("Hello There user please input Your File Name: ");
    let mut buff = String::new();
    

    // if let Err(error) = stdin().read_line(&mut buff) {
    //     return Err(error);
    // }

    stdin().read_line(&mut buff)?;

    fs::read_to_string(&buff.trim().to_lowercase())
    // let mut filee = match File::open(buff.trim().to_lowercase())  {
    //     Ok(file) => {
    //         println!("File exists");
    //         file
    //     }
    //     Err(error) => {
    //         // eprintln!("The error was {:?}",error);
    //         // process::exit(1);
    //         return Err(error);
    //     }
    // };

    // let mut file_contents = String::new();
    // File::open(buff.trim().to_lowercase())?.read_to_string(&mut file_contents)?;



    // if let Err(error)= filee.read_to_string(&mut file_contents){
    //     // eprint!("{:?}",error);
    //     // process::exit(1);
    //     return Err(error);
    // }

    // Ok(file_contents)
}


// ? operator is variant is Err, returns Err as the return value same is the case with Ok()
// ? can be used on Option too


// What value does the ? operator capture when called on an Option enum?
// The data inside Some Variant






