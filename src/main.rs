/*
#========
# name: search
# date: 2019JAN09
#       2018DEC17
#       2018DEC15
# prog: pr 
# desc: simple cli program with documentation comments
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


/// **Func** search
/// **Desc** given a <path>, search for <pattern> in the file and 
///          display the lines that contain it.
#[derive(Debug, StructOpt)]
struct Cli {
    pattern: String, 
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
    let mut located = 0;
    let mut is_found = false;

    for line in content.lines() {
        if line.contains(pattern) { 
            println!(" {:03}. {}", count, line);
            is_found = true;
            located = located + 1;
        }
        count = count + 1;
    }

    if is_found == true { 
        println!("Found pattern <{}> in {}/{} lines.", pattern, located, count);
    }

    return is_found;
}


/*
#--------
# main:
#--------
*/
fn main() {
   /// Parse arguments into cli struct.
   let args = Cli::from_args(); 

   /// Open the file content given the filepath.
   let result = std::fs::read_to_string(&args.path);

   /// Use result as enum to content or error.
   let content = match result {
       Ok (content) => content,
       Err (error) => { panic!("Error: we have an error {}, total chaos, bye.", error); }
   };

   println!("pattern:\t{:?}\nargs:\t\t{:?}\nresults:", args.pattern, args.path);
   println!("{}", content);
}



// vim: ff=unix:ts=4:sw=4:tw=78:noai:expandtab
