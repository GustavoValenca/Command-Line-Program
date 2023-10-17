use std::env;  // panics if any argument contains invalid Unicode
// use std::env::args_os  // accepts invalid Unicode. This function returns an iterator that produces OsString values instead of String values. They're more complex to work with.
use std::process;

use minigrep::Config;

fn main(){
    let args: Vec<String> = env::args().collect();  // used to make a collection. Type annotaion is required for choosing se the collection
    // dbg!(args);


    // let config = Config::build(&args);

    // // let config: Config = match config {
    // //     Ok(config) => config,
    // //     Err(erro) => {
    // //          println!("Vai falando kk {erro}");
    // //          process::exit(1)
    // //     },
    // // };

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments! {err}");
        process::exit(1);
    });


    // println!("Searching for {}", config.query);
    // println!("In file {}", config.file_path);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    };
}


