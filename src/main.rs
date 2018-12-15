/*
#========
# name: search
# date: 2018DEC15
# prog: pr 
# desc: simple cli program
# sorc: <https://rust-lang-nursery.github.io/cli-wg/tutorial/cli-args.html>
# obje: use pattern search in file
#       then display lines that are found
#========
*/


extern crate structopt;
use structopt::StructOpt;


#[derive(Debug)]
#[derive(StructOpt)]
struct Cli {
    // The pattern to look for
    pattern: String,

    // The path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::from_args();

    // display input
    println!("pattern:\t{:?}\nargs:\t\t{:?}", args.pattern, args.path);

    // open file from args
    let content = std::fs::read_to_string(&args.path)
       .expect("Warning: could not read file");
    
    // loop through the file content
    // show results OR nothing
    println!("results:");
    let mut count = 1;
    let mut is_found = false;
    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("\t\t{}. {}", count, line);
            is_found = true;
        }
        count = count + 1;
    }

    // 
    if is_found == false { println!("\t\t{:?} not found in {:?}", args.pattern, args.path); }
}

// vim: ff=unix:ts=4:sw=4:tw=78:noai:expandtab
