mcfilter
========

This is an attempt to write a program that will filter mcstas data files. It is being written 
in Rust as an exercise.

The idea is that the program will take two McStas data source files as input. One is the original
source and the second the output from a McStas instrument running with that source. Thus every
line in the second file will have originated in a line in the first file. The goal is to create
a new file that contains all the lines from the original file that resulted in output lines
in the second file. The point of this is that the resulting file will take much less time to
run that the full source but will produce the same output since the only tracks that have been
excluded are ones that would never pass through the instrument.

I need a little notation. I will all the first data file the Source file and the second file the Filter file. The intent is a command-line program with the following calling sequence

    mcfilter [-v] [-h] [-d] [<Source name> <Filter name>]

-d turns on debugging output
-v will print version information
-h will print essentially this information about the calling sequence

Note that the files may be omitted but that if the source is present then the filter is required.

The very beginning
------------------

I have created an empty Rust project with a call 

    cargo new mcfilter

and have created this *.md* file to describe the project.

Next I am going to play with code to examine command-line arguments. The Rust handbook starts
with this
```Rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(args);
}
```
which leaves the arguments in a vector of Strings. This seems a little odd to me as env::args()
is an iterator. Why don't we just use the iterator directly? What does it produce?

The documentation seems to suggest this
```Rust
// Prints each argument on a separate line
for argument in env::args() {
    println!("{argument}");
}
```
I am going to try that.
That appears to work perfectly and seems much more idiomatic to me. Though I do agree with
the book authors that it would make sense to move argument processing to a helper function
and to stuff the data into some kind of structure to return. There is one reason why the
vector method would make sense and that is that we care about the *order* of the arguments.
Let's try that way because the 0'th argument is the (currently uninteresting) program name
and the source filename must immediately precede the filter name. BUT we could also handle this
using the iterator directly. I am going to try that first.
AAAGH! It gets into issues with Some() that seem to be solved by the Vec route.
Try that way. So, this prints the arguments and knows them by number.
```Rust
fn main() {
    // Print each argument on a separate line
    let args: Vec<String> = env::args().collect();
    let pname = &args[0];
    println!("Program name = {pname:?}");
    let next_arg = &args[1];
    println!("First argument = {nextArg:?}");
    for i in 2..args.len() {
        println!("{} -> {:?}",i, args[i])
    }
    println!("Args ended");
}
```
Now, the example recommends returning the argument info in a structure and making the parse
a factory for the structure but to me that grates with the implementation of -v and -h, neither
of which are related to the config but which simply cause actions so I am going to stick with the
parse_config() route.

It took a little work to smooth out the syntax bumps but I end up with the following.
```Rust
struct Config {
    source_path: String,
    filter_path: String,
    debug: bool,
}

fn main() {
    let arg_vec: Vec<String> = env::args().collect();
    let _cfg = parse_config(&arg_vec);
    println!("Args ended");
}

fn parse_config(args: &[String]) -> Config {
    let _pname = &args[0];
    let mut name_num = 0;
    let mut files: [usize; 2] = [0, 0];
    let mut d = false;
    for i in 1..args.len() {
        match args[i].as_str() {
            "-v" => println!("Version number 0.0"),
            "-h" => print_help(),
            "-d" => d = true,
            _ => {
                if d {
                    println!("Found name {} = {:?}", name_num, args[i]);
                }
                name_num += 1;
                if name_num < 2 {
                    files[name_num] = i;
                }
            },
        }
    }
    if name_num < 2 {
        panic!{"Both source and filter file names are required."}
    }

    Config { source_path: args[files[0]].clone(),
             filter_path: args[files[1]].clone(),
             debug: d,
            }
}

//
//  Tiny helper to make argument processing clearer. Just prints the rather long help text.
//
fn print_help() {
    println!(" mcfilter [-v] [-h] [-d] [<Source name> <Filter name>]");
    println!(" -d turns on debugging output");
    println!(" -v will print version information");
    println!(" -h will print this information about the calling sequence");
}
```
I think that it is time to put this under version control.
Done.

Oh, how do I get at the cargo.toml version string?
Stackoverflow says
```Rust
const VERSION: &str = env!("CARGO_PKG_VERSION");
```
Works perfectly.

Now we should pss the rest of the work off to a processor. It will need first to
make sure that the files exist and can be opened. It will also have to make up an
output filename (I should add an argument for that!) and open that. Then it can
pass the actual work to another helper that does the actual filtering.

I am VERY tempted to make the assumption that the filter file is small enough
to be kept in memory. That will mean that the output file is also small enough.
That way I don't have to write the output out until I know how many
output lines there will be and so can write the updated header correctly from 
the start. Actually that is wrong. I only need to have read the *header* from the
Filter file to know how many lines there will be in the output file, at least if 
we succeed.

So start by writing a stuct/methods to represent a McStas data file. I plan to
do this the same way that I did in Python. I will parse every that starts
with a # and store the info in a dictionary as key (str) and value (str) pairs.
Later, I can, if needed, parse particular value strings to extract more detailed 
information.

It is probably time to start a library module to hold the McStas specific stuff.
I have created a filter_files function and moved it to the lib.rs file. I had
trouble at first because I accidentaly saved it in the wrong place. Once in the
right place it was referenced as 'use mcfilter::filter_files;' and worked fine.
I gave the error handling from the command line example in the Rust book. I should
go back and do the same sort of thing to parse_config, which currently panics if
it fails to find two names.

First let's do the basic file handling in filter_files. Its first job is to
open the two files and get them built into objects.

Because I put a reference to the file name into the McData struct I got myself
into a battle with lifetimes. It was only a matter of following the hints to
get the signatures right. I am now passing the data around correctly.

File handling is not too bad. Rust uses RAIL so that once a file handle is open,
the file is automagically closed when the handle goes out of scope.
Open either returns a file handle or an error. I found a way to use the error
and add my own information to make a better error to pass back to our caller 
if the file open failed.
```Rust
        // Note use of format! macro to do string formatting and .into() to
        // Box the result.
        let mut file = match File::open(file_name) {
            Err(why) => return Err(format!("couldn't open {}: {}", file_name, why).into()),
            Ok(file) => file,
        };
```
Hm, I get into trouble trying to read the comment lines with one for loop and then
the data lines with another like this
```Rust
        let data_lines = io::BufReader::new(file).lines();
        for line in data_lines {
            if let Ok(this_line) = line {
                if this_line.starts_with("# ") {
                    println!("Comment {}", this_line);
                } else {
                    break;
                }
            }
        }
        //
        // Read on to eat the data too.
        //
        for line in data_lines {
            if let Ok(this_line) = line {
                println!("Data {}", this_line);
            }
        }
```
It runs into the following compiler errors.
```
error[E0382]: use of moved value: `data_lines`
   --> src/lib.rs:41:21
    |
27  |         let data_lines = io::BufReader::new(file).lines();
    |             ---------- move occurs because `data_lines` has type `std::io::Lines<BufReader<File>>`, which does not implement the `Copy` trait
28  |         for line in data_lines {
    |                     ---------- `data_lines` moved due to this implicit call to `.into_iter()`
...
41  |         for line in data_lines {
    |                     ^^^^^^^^^^ value used here after move
    |
note: this function takes ownership of the receiver `self`, which moves `data_lines`
   --> /Users/bcollett/.rustup/toolchains/stable-x86_64-apple-darwin/lib/rustlib/src/rust/library/core/src/iter/traits/collect.rs:262:18
    |
262 |     fn into_iter(self) -> Self::IntoIter;
    |                  ^^^^

For more information about this error, try `rustc --explain E0382`.
```
Attempts to get round it with a reference do not work.
I *can* do it by testing for comments and data within the same loop but that means
that I can't return the file in a state ready to read the next line.
The answer may be explicit calls to .next() rather the for construct.
This took a minute to get right because there is rather a lot of r=wrapping going on. 
Here is what works.
```Rust
        //  NOTE that the iterator reetursn an object of type
        //  Option<Result<String, std::io::Error>> so that we have to
        //  destructure it TWICE.
        //
        let mut data_lines = io::BufReader::new(file).lines();
        loop {
            let line = data_lines.next();
            if let Some(a_line) = line {
                if let Ok(this_line) = a_line {
                    if this_line.starts_with("# ") {
                        println!("Comment {}", this_line);
                    } else {
                        println!("FOund non-comment line {}", this_line);
                        break;
                    }
                } else {
                    println!("Destructure failed!");
                    break;
                }
            }
        }
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
```
I do feel that there should be a better to do the de-structuring. I am not clear why
we have the double wrap.
That had one error. It lost the first non-comment line. I used a pre-declaration to
move that first non-comment line into a variable with function scope so that I could
access it after processing the comment lines. I do think that there should be a rustier
way of doing some of this. It looks like a lot of very procedural code.

The next improvement would be to parse the comment lines and enter them in a dictionary.
Rust has the HashMap that seems to provide that functionality. The problem I forsee is
issues with ownership. Let's see if I run into anything nasty.

