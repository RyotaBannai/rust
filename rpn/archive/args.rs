fn use_buildin_methods() {
  use std::env;
  let args: Vec<String> = env::args().collect();
  println!("{:?}", args)
}

fn use_builder_ptn() {
  use clap::{App, Arg};
  let mathces = App::new("Name: My RPN program.")
    .version("1.0.0")
    .author("Author: Ryota, ryotala0528@gmail.com")
    .about("About: Super awesome RPN calculator")
    .arg(
      Arg::new("formula_file")
        .about("Formulas written in RPN")
        .value_name("FILE")
        .index(1)
        .required(false),
    )
    .arg(
      Arg::new("verbose")
        .about("Sets the level of verbosity")
        .short('v')
        .long("verbose")
        .required(false),
    )
    .get_matches();

  match mathces.value_of("formula_file") {
    Some(file) => println!("File specified: {}", file),
    None => println!("No file specified."),
  }

  let verbose = mathces.is_present("verbose");
  println!("Is verbosity specified?: {}", verbose);
}
