use std::env;

use mcfilter::filter_files;
//
//  Tie our version to the one in Cargo.toml
//
const VERSION: &str = env!("CARGO_PKG_VERSION");

//
// Filled in from cmd. line arguments.
//
struct Config {
    source_path: String,
    filter_path: String,
    debug: bool,
}

fn main() {
    let arg_vec: Vec<String> = env::args().collect();
    let cfg = parse_config(&arg_vec);
    println!("Args ended");
    if let Err(e) = filter_files(&cfg.source_path, &cfg.filter_path) {
        println!("Filter reported error: {e}");
    } else {
        if cfg.debug {
            println!("Success!");
        }
    }
}

//
//  Parse the arguments into the config struct.
//  Note that we also handle the -v and -h args that do not affect the config.
//  We error out if we don't find both file names.
//
fn parse_config(args: &[String]) -> Config {
    let _pname = &args[0];
    let mut name_num = 0;
    let mut files: [usize; 2] = [0, 0];
    let mut d = false;
    for i in 1..args.len() {
        match args[i].as_str() {
            "-v" => println!("Version number {}", VERSION),
            "-h" => print_help(),
            "-d" => d = true,
            _ => {
                if d {
                    println!("Found name {} = {:?}", name_num, args[i]);
                }
                if name_num < 2 {   // Track names in order
                    files[name_num] = i;
                    name_num += 1;
                }
            },
        }
    }
    if name_num < 2 {
        panic!{"Both source and filter file names are required."}
    } else {
        println!("{:?}", files);
    }
    //
    // Pack info into a config that we return. Note we clone strings
    // to avoid ownership issues. The cost is very small.
    //
    Config { source_path: args[files[0]].clone(),
             filter_path: args[files[1]].clone(),
             debug: d,
            }
}

//
//  Tiny helper to make argument processing clearer. 
//  Just prints the rather long help text.
//
fn print_help() {
    println!(" mcfilter [-v] [-h] [-d] [<Source name> <Filter name>]");
    println!(" -d turns on debugging output");
    println!(" -v will print version information");
    println!(" -h will print this information about the calling sequence");
}