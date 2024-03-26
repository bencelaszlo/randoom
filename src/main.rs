mod generator;
mod io;

extern crate rand;

extern crate clap;
use clap::{Arg, ArgAction, Command};

extern crate colored;
use colored::*;

extern crate serde;
extern crate serde_json;

extern crate regex;

fn print_in_verbose_mode(verbose_mode: bool, text: &str, text_color: &str) {
    if verbose_mode {
        match text_color {
            "white" => print!("{}", text.white()),
            "black" => print!("{}", text.black()),
            "red" => print!("{}", text.red()),
            "green" => print!("{}", text.green()),
            "yellow" => print!("{}", text.yellow()),
            "blue" => print!("{}", text.blue()),
            "magenta" => print!("{}", text.magenta()),
            "cyan" => print!("{}", text.cyan()),
            _ => print!("{}", text.white()),
        }
    }
}

fn main() -> std::io::Result<()> {
    let matches = Command::new("randoom")
             .version("0.4.0")
             .author("Bence László <bence@bencelaszlo.hu>")
             .about("Generate random numbers, texts, JSONs and colors.")

	         .arg(Arg::new("verbose")
	         .help("Verbose mode. Print more detail to the terminal.")
	         .short('v')
	         .long("verbose")
             .action(ArgAction::SetTrue)
	         .required(false))

	         .arg(Arg::new("datatype")
	         .help("Choose the type of the generated data. Possible options: f32, f64, u32, u64, usize, i32, i64, isize, rgb.")
	         .short('t')
	         .long("datatype")
             .action(ArgAction::Set)
	         .num_args(1)
	         .required(true))

	         .arg(Arg::new("number")
	         .help("Number of data to be generated.")
	         .short('n')
	         .long("number")
	         .num_args(1)
	         .required(true))

	         .arg(Arg::new("lower_limit")
	         .help("Lower limit for numbers.")
	         .long("lower")
             .value_name("FILE")
             .action(ArgAction::Set)
	         .num_args(1)
	         .required(false))

	         .arg(Arg::new("higher_limit")
	         .help("Higher limit for numbers.")
	         .long("higher")
             .action(ArgAction::Set)
	         .num_args(1)
	         .required(false))

	         .arg(Arg::new("separator")
	         .help("Separator character between individual values. Default value: \n (new line).")
	         .short('s')
	         .long("separator")
             .action(ArgAction::Set)
	         .num_args(1)
             .required(false))

             .arg(Arg::new("special_character_mode")
             .help("Turn on special characters in character generator.")
             .long("special-character-mode")
             .action(ArgAction::SetTrue)
             .required(false))

             .arg(Arg::new("number_character_mode")
             .help("Turn on numbers in character generator.")
             .long("number-character-mode")
             .action(ArgAction::SetTrue)
             .required(false))

             .arg(Arg::new("string_min_length")
             .help("Minimum length of random strings.")
             .long("string-min-length")
             .action(ArgAction::Set)
             .required(false))

             .arg(Arg::new("string_max_length")
             .help("Maximum length of random strings.")
             .long("string-max-length")
             .action(ArgAction::Set)
             .required(false))

	         .arg(Arg::new("output")
	         .help("Output filename. Default value: random_data.txt")
	         .short('o')
	         .long("output")
             .action(ArgAction::Set)
	         .required(false))

	         .get_matches();

    println!(
        "{}{}{}{}{}{}{}",
        "r".red().blink().underline().bold(),
        "a".yellow().blink().underline().bold(),
        "n".white().blink().underline().bold(),
        "d".cyan().blink().underline().bold(),
        "o".blue().blink().underline().bold(),
        "o".green().blink().underline().bold(),
        "m".magenta().blink().underline().bold()
    );

    let option_verbose: bool = matches.get_one::<bool>("verbose").unwrap().clone();
    let mut option_datatype: String = "".to_string();
    let mut option_number: usize = 0;
    let mut option_lower_limit: f64 = 0.0f64;
    let mut option_higher_limit: f64 = 1.0f64;
    let mut option_separator: char = "\n".parse().unwrap();
    let option_special_character_mode: bool = matches.get_one::<bool>("special_character_mode").unwrap().clone();
    let option_number_character_mode: bool = matches.get_one::<bool>("number_character_mode").unwrap().clone();
    let mut option_string_min_length: usize = 0;
    let mut option_string_max_length: usize = 4;
    let mut option_output_filename: String = "./randoom_data.txt".to_string();

    if option_verbose {
        print_in_verbose_mode(
            option_verbose,
            "randoom will operate in verbose mode.",
            "white",
        );
    }

    print_in_verbose_mode(
        option_verbose,
        "\nGenerate random data with the following parameters:",
        "cyan",
    );

    /*if let Some(json) = app.value_of("json") {
        option_json = json.to_string();
    }*/

    if let Some(datatype) = matches.get_one::<String>("datatype") {
        print_in_verbose_mode(option_verbose, "\nChosen type: ", "white");
        print_in_verbose_mode(option_verbose, datatype, "cyan");
        option_datatype = datatype.to_string();
    }

    if let Some(number) = matches.get_one::<String>("number") {
         print_in_verbose_mode(option_verbose, "\nNumber: ", "white");
        print_in_verbose_mode(option_verbose, number, "cyan");
        option_number = number.parse().unwrap();
    }

    if let Some(lower_limit) = matches.get_one::<String>("lower_limit") {
        print_in_verbose_mode(option_verbose, "\nLower limit: ", "white");
        print_in_verbose_mode(option_verbose, lower_limit, "cyan");
        option_lower_limit = lower_limit.parse().unwrap();
    }

    if let Some(higher_limit) = matches.get_one::<String>("higher_limit") {
        print_in_verbose_mode(option_verbose, "\nHigher limit: ", "white");
        print_in_verbose_mode(option_verbose, higher_limit, "cyan");
        option_higher_limit = higher_limit.parse().unwrap();
    }

    if option_special_character_mode {
        print_in_verbose_mode(
            option_verbose,
            "\nCharacter generator can generate special characters now, too.",
            "white",
        );
    }

    if option_number_character_mode {
        print_in_verbose_mode(
            option_verbose,
            "\nCharacter generator can generate number characters now, too.",
            "white",
        );
    }

    if let Some(string_min_length) = matches.get_one::<String>("string_min_length") {
        print_in_verbose_mode(option_verbose, "\nminimum length: ", "white");
        print_in_verbose_mode(option_verbose, string_min_length, "cyan");
        option_string_min_length = string_min_length.parse().unwrap();
    }

    if let Some(string_max_length) = matches.get_one::<String>("string_max_length") {
        print_in_verbose_mode(option_verbose, "\nmaximum length: ", "white");
        print_in_verbose_mode(option_verbose, string_max_length, "cyan");
        option_string_max_length = string_max_length.parse().unwrap();
    }

    if let Some(separator) = matches.get_one::<String>("separator") {
        print_in_verbose_mode(option_verbose, "\nseparator:", "white");
        print_in_verbose_mode(option_verbose, separator, "cyan");
        option_separator = separator.parse().unwrap();
    }

    if let Some(output) = matches.get_one::<String>("output") {
        print_in_verbose_mode(option_verbose, "\noutput filename: ", "white");
        print_in_verbose_mode(option_verbose, output, "cyan");
        option_output_filename = output.parse().unwrap();
    }

    print_in_verbose_mode(
        option_verbose,
        "\nGenerate random data and write generated data into file...",
        "magenta",
    );
    print_in_verbose_mode(option_verbose, "\nCreate file...", "magenta");

    print_in_verbose_mode(option_verbose, "\nGenerate random values...\n", "magenta");

    if option_datatype == "f32" || option_datatype == "float" || option_datatype == "real" {
        let random_data: Vec<f32> = generator::number_generator(
            option_number,
            option_lower_limit as f32,
            option_higher_limit as f32,
            option_verbose,
        );
        let _file_write_result =
            io::write_numbers_to_file(random_data, option_output_filename, option_separator);
    } else if option_datatype == "f64" || option_datatype == "double" {
        let random_data: Vec<f64> = generator::number_generator(
            option_number,
            option_lower_limit,
            option_higher_limit,
            option_verbose,
        );
        let _file_write_result =
            io::write_numbers_to_file(random_data, option_output_filename, option_separator);
    } else if option_datatype == "i32" || option_datatype == "int" || option_datatype == "integer" {
        let random_data: Vec<i32> = generator::number_generator(
            option_number,
            option_lower_limit as i32,
            (option_higher_limit + 1.0f64) as i32,
            option_verbose,
        );
        let _file_write_result =
            io::write_numbers_to_file(random_data, option_output_filename, option_separator);
    } else if option_datatype == "isize" {
        let random_data: Vec<isize> = generator::number_generator(
            option_number,
            option_lower_limit as isize,
            (option_higher_limit + 1.0f64) as isize,
            option_verbose,
        );
        let _file_write_result =
            io::write_numbers_to_file(random_data, option_output_filename, option_separator);
    } else if option_datatype == "u32" {
        let random_data: Vec<u32> = generator::number_generator(
            option_number,
            option_lower_limit as u32,
            (option_higher_limit + 1.0f64) as u32,
            option_verbose,
        );
        let _file_write_result =
            io::write_numbers_to_file(random_data, option_output_filename, option_separator);
    } else if option_datatype == "u64" {
        let random_data: Vec<u64> = generator::number_generator(
            option_number,
            option_lower_limit as u64,
            (option_higher_limit + 1.0f64) as u64,
            option_verbose,
        );
        let _file_write_result =
            io::write_numbers_to_file(random_data, option_output_filename, option_separator);
    } else if option_datatype == "usize" {
        let random_data: Vec<usize> = generator::number_generator(
            option_number,
            option_lower_limit as usize,
            (option_higher_limit + 1.0f64) as usize,
            option_verbose,
        );
        let _file_write_result =
            io::write_numbers_to_file(random_data, option_output_filename, option_separator);
    } else if option_datatype == "char" || option_datatype == "character" {
        let random_data: Vec<char> = generator::char_generator(
            option_number,
            option_special_character_mode,
            option_number_character_mode,
            option_verbose,
        );
        let _file_write_result =
            io::write_to_file(random_data, option_output_filename, option_separator);
    } else if option_datatype == "string" || option_datatype == "str" {
        let random_data: Vec<String> = generator::string_generator(
            option_number,
            option_string_min_length,
            option_string_max_length,
            option_special_character_mode,
            option_number_character_mode,
            option_verbose,
        );
        let _file_write_result =
            io::write_to_file(random_data, option_output_filename, option_separator);
    } else if option_datatype == "color" || option_datatype == "rgb" {
        let random_data = generator::color_generator(option_number, option_verbose);
        let _file_write_result =
            io::write_to_file(random_data, option_output_filename, option_separator);
    } else if option_datatype == "bool"
        || option_datatype == "boolean"
        || option_datatype == "logic"
    {
        let random_data = generator::bool_generator(option_number, option_verbose);
        let _file_write_result =
            io::write_to_file(random_data, option_output_filename, option_separator);
    }

    print_in_verbose_mode(option_verbose, "\n ---> ", "white");
    print_in_verbose_mode(
        option_verbose,
        &String::from(option_number.to_string()).bold(),
        "green",
    );

    if option_verbose {
        println!(
            "\n-----> {} {}",
            option_number.to_string().green().bold(),
            " data generated and wrote into file successfully.".green()
        );
    }

    Ok(())
}
