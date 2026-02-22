use std::error::Error;

use clap::{Arg, ArgMatches, ArgAction, arg, Command};

type MyResult<T> = Result<T, Box<dyn Error>>;


#[derive(Debug)]
// This represents the command-line parameters definitions
pub struct Config {
    files: Vec<String>,

    number_lines: bool,

    number_nonblank_lines: bool,
}

// A function to instantiate the user's arguments
pub fn get_user_args() -> MyResult<Config> {

    // instantiate the binary
    let catr_details: ArgMatches = Command::new("catr")
        .author("0xweebad, weebadn@proton.me")
        .about("My own implementation of the cat command-line linux utility")
        .version("0.1.0")
        .long_version("This is the first version of this catr implementation")
        //.arg() // Add Arguments later
        .arg(
            Arg::new("file")
                .value_name("FILES")
                .num_args(1..)
                .required(true)
                .help("This argument indicates words to be passed")
        )
        .arg(
            arg!(-n --number_lines <NUMBER_OF_LINES> "this flag indicates whether or not to print the line numbers")
        )
        // .arg(
        //     arg!(-i --include_nonblank_lines <NON-BLANK-LINES> "This flag indicates whether to include a blank line or not")
        // )
        .arg(
            Arg::new("include_unblank_lines")
                .short('i')
                .long("include_nonblank_lines")
                .required(false)
                .action(ArgAction::SetTrue)
                .help("This flag includes a non blank line when set")
        )
        .get_matches();

        // Get the various command-line arguments and instantiate the struct below
        let file_arg: Vec<_> = catr_details.get_many::<String>("file")
            .expect("required")
            .cloned()
            //.map(|s| s.as_str())
            .collect();

    Ok(Config {
        files: file_arg,
        number_lines: catr_details.get_flag("number_lines"),
        number_nonblank_lines: catr_details.get_flag("include_unblank_lines"),
    })
}

pub fn run(config: Config) -> MyResult<()> {

    dbg!(config);

    //println!("Hello, world!");

    Ok(())
}