// use std::env;
use clap::{App, Arg};

fn main() {
    // let args: Vec<String> = env::args().collect();
    // println!("{:?}", args)

    let mathces = App::new("My RPN program.")
        .version("1.0.0")
        .author("Ryota")
        .about("Super awesome RPN calculator")
        .arg(
            Arg::with_name("formula_file")
                .help("Formulas written in RPN")
                .value_name("FILE")
                .index(1)
                .required(false),
        )
        .arg(
            Arg::with_name("verbose")
                .help("Sets the level of verbosity")
                .value_name("v")
                .long("verbose")
                .required(false),
        )
        .get_matches();

    match mathces.value_of("formula_file") {
        Some(file) => println!("File specified: {}", file),
        None => println!("No file specified."),
    }

    let verbose = mathces.is_present("verbose");
    println!("Is verbosity specified?:{} ", verbose);
}
