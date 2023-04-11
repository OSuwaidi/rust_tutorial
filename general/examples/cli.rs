// بسم الله الرحمن الرحيم وبه نستعين

use std::env::args;
// This is importing the "args" *struct* from the environment (env) module from the standard (std) module
use std::fs::{read_to_string, write};
use std::io::Result;

fn main() -> Result<()> {
    // "Result<()>" <-> "Result<(), Error>"
    let mut args = args().skip(1); // this allows you insert command line arguments as "-- args" in the terminal when running a file
    let key = args.next().expect("The key is missing");
    let value = args.next().expect("The value is missing");
    /*
    ".skip(n)" skips "n" elements from the "args" iterator, where the first argument is the hidden path from terminal
    ".next()" retrieves the next element from the "args" iterator
    ".unwrap()" extracts and returns the value of the argument
    ".expect()" same as ".unwrap()" but returns an error message instead of "panic" in case of error
    */
    println!("The key is '{}' and the value is '{}'", key, value);
    let contents = format!("{}\t{}\n", key, value); // the "format!" marco returns a "String" datatype

    // "write" creates a ".db" file, and returns a "Result<()>" *enum* (that can be unpacked/extracted)
    let did_write = write("mydata.db", contents);
    /*
    "write()" will overwrite the file if exits. Therefore, if we wanted to only update by appending to the end of the file,
    we should *read* the file first, copy over all its contents (store in a HashMap), then *write* a new file and *push* over all
    the old contents via "push_str()", then finally, insert the new values into the db
     */
    let write_result = match did_write {
        // If "write_result" is of type variant "Ok":
        Ok(()) => {
            println!("File successfully created");
            1 // if we "return" here, it will stop the function early and return upon success
        },

        // If "write_result" is type variant "Err":
        Err(error) => return Err(error),
    };

    /*
    *** The "match" case above, where you "match" the "Result" type to extract the result of the "Ok" case
    and bind to the variable "write_result", or returning the "Err" case back from the function;
    that can be entirely summarized with the "?" operator ***
    */
    let contents = read_to_string("mydata.db")?; // the "?" at the end means that the function on the left will contain error handling
    for line in contents.lines() {
        let pair = line.split_once("\t").expect("Corrupt database");
        println!("Database pair: {:?}", pair);
    }
    return Ok(());
}
