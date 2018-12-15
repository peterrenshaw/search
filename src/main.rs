/*
#========
# name: search
# date: 2018DEC15
# prog: pr 
# desc: simple cli program
# sorc: <https://rust-lang-nursery.github.io/cli-wg/tutorial/cli-args.html>
# obje: read pattern, file then search in teh contents of the file.
#========
*/


extern crate structopt;
use structopt::StructOpt;


// pattern search in file
// display lines that contain it
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


    println!("Pattern:\t<{:?}>\nargs:\t\t<{:?}>", args.pattern, args.path);
}

// vim: ff=unix:ts=4:sw=4:tw=78:noai:expandtab
