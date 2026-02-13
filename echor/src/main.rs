
use clap::{Arg, ArgMatches, ArgAction, Command, arg};

fn main() {
    // println!("Hello, world!");
    // @dev The below function call reads the command line arguments passed
    // treats them as separate args using space
   // println!("{:?}",std::env::args());
    // @dev as at this point, try passing a flag "-f" results in unexpected argument

    //@dev Utilize "clap" for parsing command-line arguments

    let details: ArgMatches = Command::new("echor")
        .author("Ernest Baffoe, ernest.baffoe@amalitech.com")
        .version("0.1.0")
        .long_version("
        semantic versioning is supported for
         this implementation")
        .about("My own implementation for the echo utility")
        //.trailing_var_arg(true)
        .arg(
            Arg::new("text")
                .value_name("TEXT")
                //.short('t') Positional Arguments Don't Take The "SHORT" or "LONG" method
                .num_args(1..)
                .required(true)
                .help("This flag turns on text mode")
        )
        .arg(
            Arg::new("omit_newline")
                .short('n')// This turns it into a FLAG/OPTION
                .help("This flag omits a newline")
                .action(ArgAction::SetTrue)
                .required(false)
        )
        .arg(
            arg!(-c --config <CONFIG> "Optionally sets a config file to use")
        )
        .arg(
            arg!(-d --debug <DEBUG> "Optionally turns the parsing in debug mode")
        )
        .arg(
            arg!(-f --force "Optionally uses the force bit to force execution")
        )
        .get_matches();


    // Print the matches
    //println!("{:#?}", details);

    // Creating the Output
    //@dev Redirect to STDOUT for non-error, and errors get redirected to STDERR
    //@dev Used to get information about the arguments that were supplied 
    //to the program at runtime by the user

    // Know id for text, and omit_newline
    // Get Config command
    //let text_position = details.get_one::<String>("text").expect("required");
    //println!("{:?}", text_position);

    //let raw_values:Vec<String> = details.get_many/*::<String>*/("text").expect("required").copied().collect()
    //let config_extract = details.get_one::<String>("config").unwrap();
    let raw_values: Vec<_> = details.get_many::<String>("text")
        .expect("required")
        .map(|s| s.as_str())
        .collect();

    let omit_newline = details.get_flag("omit_newline");
    //let add_line = if omit_newline {" "} else { "\n" };
    let final_values = raw_values.join(" ");
    if omit_newline {
        print!("{final_values}");
    } else {
        println!("{final_values}");
    }
        

}
