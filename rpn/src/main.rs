use clap::Clap;

// use macro pattern
#[derive(Clap, Debug)]
#[clap(
    name = "Name: My RPN program.",
    version = "1.0.0",
    author = "Author: Ryota, ryotala0528@gmail.com",
    about = "About: Super awesome RPN calculator"
)]
struct Opts {
    /// Sets the level of verbosity
    #[clap(short, long)]
    verbose: bool,

    /// Formulas written in RPN
    #[clap(name = "FILE")]
    formula_file: Option<String>,
}
fn main() {
    let opts = Opts::parse();
    match opts.formula_file {
        Some(file) => println!("File specified: {}", file),
        None => println!("No file specified."),
    }
    println!("Is verbosity specified?: {}", opts.verbose);
}
