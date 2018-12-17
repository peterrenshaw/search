/*
#========
# name: search
# date: 2018DEC17
#       2018DEC15
# prog: pr 
# desc: simple cli program
# obje: use pattern search in file then display lines that are found
# usag:  
#           key:      value you want to search for in file
#           filepath: valid filepath to file
#
#           ./search key some/given/a/filepath/file.dat
#
# sorc: <https://rust-lang-nursery.github.io/cli-wg/tutorial/cli-args.html>
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

/*
#--------
# show_result: is search key in content?
#              search line by line to check if pattern 
#              is in a line, display, move to next line.
#              if found, return t/f
# args:      
#              pattern: key as string to find in content.
#              content: string separated by lines to search.
#
# return:      is pattern found as bool
#--------
*/
fn show_result(pattern: &str, content: &std::string::String) -> bool {
    let mut count = 0;
    let mut is_found = false;

    for line in content.lines() {
        if line.contains(pattern) {
            println!("\t\t{}. {}", count, line);
            is_found = true;
        }
        count = count + 1;
    }

    return is_found;
}


/*
#--------
# main:
#--------
*/
fn main() {
    let args = Cli::from_args();

    println!("results:");
    println!("pattern:\t{:?}\nargs:\t\t{:?}", args.pattern, args.path);

    // open file from args
    let result = std::fs::read_to_string(&args.path);
    let content = match result {
        Ok(content) => { content },
        Err(error)  => { panic!("Error: we have an error {}, total chaos, bye.", error); }
    };



    // TODO put this in a function
    // if key not found, tell user
    if show_result(&args.pattern, &content) == false {
        println!("\t\t{:?} not found in {:?}", args.pattern, args.path);
    }
}

// vim: ff=unix:ts=4:sw=4:tw=78:noai:expandtab
