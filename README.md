README


Objective: Simple program to read CLI arguments from command line with search functionality.
<https://rust-lang-nursery.github.io/cli-wg/tutorial/cli-args.html>

* take two arguments: <key> <filepath>

* return line if found, else fails


2018DEC15
* handle the errors for reading args.path using an enum, then match result.

* this is nice: search for keyword in file, for example this search
  is through the source code looking for refernces to mutability. Result
  then tells you which line the reference is.

```
 search pr$ cargo run mut src/main.rs
    Finished dev [unoptimized + debuginfo] target(s) in 0.06s                                             
     Running `target/debug/search mut src/main.rs`
 pattern:	"mut"
 args:		"src/main.rs"
 results:
		42.     let mut count = 1;
		43.     let mut is_found = false;
```

* added line numbers, message if no key in file.

* example:

```
cargo run n Cargo.toml 
```

yields

```
pattern:	<"n">
args:		<"Cargo.toml">
results:
		name = "search"
		version = "0.1.0"
		authors = ["Peter Renshaw <peterrenshaw@seldomlogical.com>"]
		[dependencies]
```



* remember Cargo.toml has structopt = "0.2.10"
* simple example working
* added README, LICENCE
* checked in.

* read <https://help.github.com/articles/adding-a-remote/> 
