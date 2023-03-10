use std::error::Error;
use std::result::Result::Err;
use std::fs::File;
use std::io::{self, BufRead};

struct McData<'a> {
    file_name: &'a String,
}

impl<'a> McData<'a> {
    fn build<'b>(file_name: &'b String) -> Result<McData, Box<dyn Error>> {
        println!("Build data from file {}", file_name);

        // Open the path in read-only mode, returns `io::Result<File>`
        // File automatically closed when variable leaves scope.
        // Use long form rather than ? form to make error message more specific.
        // Note use of format! macro to do string formatting and .into() to
        // Box the result.
        let file = match File::open(file_name) {
            Err(why) => return Err(format!("couldn't open {}: {}", file_name, why).into()),
            Ok(file) => file,
        };
        //
        //  File is open. Read through, line by line, until we run out of header.
        //  This is done with an iterator that we call 'data_lines'.
        //  NOTE that the iterator reetursn an object of type
        //  Option<Result<String, std::io::Error>> so that we have to
        //  destructure it TWICE.
        //
        let mut data_lines = io::BufReader::new(file).lines();
        let next_line;
        loop {
            let line = data_lines.next();
            if let Some(a_line) = line {
                if let Ok(this_line) = a_line {
                    if this_line.starts_with("# ") {
                        println!("Comment {}", this_line);
                    } else {
                        println!("FOund non-comment line {}", this_line);
                        next_line = this_line;
                        break;
                    }
                } else {
                    next_line = "".to_string();
                    println!("Destructure failed!");
                    break;
                }
            }
        }
        println!("Exit non-comment line {}", next_line);
        //
        // Read on to eat the data too.
        //
        loop {
            let line = data_lines.next();
            if let Some(a_line) = line {
                if let Ok(this_line) = a_line {
                    if this_line.starts_with("# ") {
                        println!("Comment in data {}", this_line);
                    } else {
                        println!("Data {}", this_line);
                    }
                }
            } else {
                println!("Destructure failed!");
                break;
            }
        }
        Ok(McData { file_name })
    }
}

pub fn filter_files<'m>(src_name: &'m String, filt_name: &'m String) -> Result<(), Box<dyn Error>> {
    // Needs a body
    let src = McData::build(src_name)?; // Returns error if fails
    let filt = McData::build(filt_name)?; // Returns error if fails
    println!("filter_file {} with {}", src.file_name, filt.file_name);
    Ok(())
}

