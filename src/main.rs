use std::env;

fn main() {
    // get/accept arguments from the command line
    let args: Vec<String> = env::args().collect();
    // save argument values into variables
    let query = &args[1];
    let file_path = &args[2];

    // print out the saved variables
    println!("Searching for {query}");
    println!("In file {file_path}");
}
