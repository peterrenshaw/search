README


Objective: Simple program to read CLI arguments from command line with search functionality.
<https://rust-lang-nursery.github.io/cli-wg/tutorial/cli-args.html>

* take two arguments: <key> <filepath>

* return line if found, else fails


2018DEC15
* added line numbers, message if no key in file.

* example:

```
cargo run n Cargo.toml 
```

yeilds

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
