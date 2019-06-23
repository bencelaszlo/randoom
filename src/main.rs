mod io;
mod generator;

use std::fs::File;
use std::io::Write;

extern crate rand;
use rand::Rng;

extern crate clap;
use clap::{App, AppSettings, Arg};

extern crate colored;
use colored::*;

extern crate serde;
extern crate serde_json;

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
            _ => print!("{}", text.white())
        }
    }
}

fn main() -> std::io::Result<()> {

    let app = App::new("randoom")
             .setting(AppSettings::ColorAuto)
			 .setting(AppSettings::AllowNegativeNumbers)
             .version("0.2.0")
             .author("Bence László <bencelaszlo@protonmail.com>")
             .about("Generate random numbers and colors.")

	         .arg(Arg::with_name("verbose")
	         .help("Verbose mode. Print more detail to the terminal.")
	         .short("v")
	         .long("verbose")
	         .takes_value(false)
	         .required(false))

	         .arg(Arg::with_name("datatype")
	         .help("Choose the type of the generated data. Possible options: f32, f64, u32, u64, usize, i32, i64, isize, rgb.")
	         .short("t")
	         .long("datatype")
	         .takes_value(true)
	         .required(true))

	         .arg(Arg::with_name("number")
	         .help("Number of data to be generated.")
	         .short("n")
	         .long("number")
	         .takes_value(true)
	         .required(true))

	         .arg(Arg::with_name("lower_limit")
	         .help("Lower limit for numbers.")
	         .short("l")
	         .long("lower")
	         .takes_value(true)
	         .required(false))

	         .arg(Arg::with_name("higher_limit")
	         .help("Higher limit for numbers.")
	         .short("h")
	         .long("higher")
	         .takes_value(true)
	         .required(false))

	         .arg(Arg::with_name("separator")
	         .help("Separator character between individual values. Default value: \n (new line).")
	         .short("s")
	         .long("separator")
	         .takes_value(true)
	         .required(false))

	         .arg(Arg::with_name("output")
	         .help("Output filename. Default value: random_data.txt")
	         .short("o")
	         .long("output")
	         .takes_value(true)
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

    let mut option_verbose: bool = false;
    let mut option_datatype: String = "".to_string();
    let mut option_number: usize = 0;
    let mut option_lower_limit: f64 = 0.0f64;
    let mut option_higher_limit: f64 = 1.0f64;
    let mut option_separator: char = "\n".parse().unwrap();
    let mut option_output_filename: String = "./random_data.txt".to_string();

    if app.is_present("verbose") {
        option_verbose = true;
        print_in_verbose_mode(option_verbose, "randoom will operate in verbose mode.", "white");
    }

	print_in_verbose_mode(option_verbose, "\nGenerate random data with the following parameters:", "cyan");

    if let Some(datatype) = app.value_of("datatype") {
        print_in_verbose_mode(option_verbose, "\nchoosen type:", "white");
		print_in_verbose_mode(option_verbose, datatype, "cyan");
        option_datatype = datatype.to_string();
    }

    if let Some(number) = app.value_of("number") {
        print_in_verbose_mode(option_verbose, "\nnumber: ", "white");
        print_in_verbose_mode(option_verbose, number, "cyan");
        option_number = number.parse().unwrap();
    }

    if let Some(lower_limit) = app.value_of("lower_limit") {
        print_in_verbose_mode(option_verbose, "\nlower limit: ", "white");
        print_in_verbose_mode(option_verbose, lower_limit, "cyan");
        option_lower_limit = lower_limit.parse().unwrap();
    }

    if let Some(higher_limit) = app.value_of("higher_limit") {
        print_in_verbose_mode(option_verbose, "\nhigher limit: ", "white");
        print_in_verbose_mode(option_verbose, higher_limit, "cyan");
        option_higher_limit = higher_limit.parse().unwrap();
    }

    if let Some(separator) = app.value_of("separator") {
        print_in_verbose_mode(option_verbose, "\nseparator:", "white");
        print_in_verbose_mode(option_verbose, separator, "cyan");
        option_separator = separator.parse().unwrap();
    }

    if let Some(output) = app.value_of("output") {
        print_in_verbose_mode(option_verbose, "\noutput filename: ", "white");
        print_in_verbose_mode(option_verbose, output, "cyan");
        option_output_filename = output.parse().unwrap();
    }

    print_in_verbose_mode(option_verbose, "\nGenerate random data and write generated data into file...", "magenta");
    print_in_verbose_mode(option_verbose, "\nCreate file...", "magenta");

    print_in_verbose_mode(option_verbose, "\nGenerate random values...\n", "magenta");

    if option_datatype == "f32" || option_datatype == "float" || option_datatype == "real" {
        let random_data: Vec<f32> = generator::number_generator(option_number, option_lower_limit as f32, option_higher_limit as f32, option_verbose);
        let _file_write_result = io::write_numbers_to_file(random_data, option_output_filename, option_separator);
    } else if option_datatype == "f64" || option_datatype == "double" {
        let random_data: Vec<f64> = generator::number_generator(option_number, option_lower_limit, option_higher_limit, option_verbose);
        let _file_write_result = io::write_numbers_to_file(random_data, option_output_filename, option_separator);
    } else if option_datatype == "i32" || option_datatype == "int" || option_datatype == "integer" {
        let random_data: Vec<i32> = generator::number_generator(option_number, option_lower_limit as i32, (option_higher_limit + 1.0f64) as i32, option_verbose);
        let _file_write_result = io::write_numbers_to_file(random_data, option_output_filename, option_separator);
    } else if option_datatype == "isize" {
        let random_data: Vec<isize> = generator::number_generator(option_number, option_lower_limit as isize, (option_higher_limit + 1.0f64) as isize, option_verbose);
        let _file_write_result = io::write_numbers_to_file(random_data, option_output_filename, option_separator);
    } else if option_datatype == "u32" {
        let random_data: Vec<u32> = generator::number_generator(option_number, option_lower_limit as u32, (option_higher_limit + 1.0f64) as u32, option_verbose);
        let _file_write_result = io::write_numbers_to_file(random_data, option_output_filename, option_separator);
    } else if option_datatype == "u64" {
        let random_data: Vec<u64> = generator::number_generator(option_number, option_lower_limit as u64, (option_higher_limit + 1.0f64) as u64, option_verbose);
        let _file_write_result = io::write_numbers_to_file(random_data, option_output_filename, option_separator);
    } else if option_datatype == "usize" {
        let random_data: Vec<usize> = generator::number_generator(option_number, option_lower_limit as usize, (option_higher_limit + 1.0f64) as usize, option_verbose);
        let _file_write_result = io::write_numbers_to_file(random_data, option_output_filename, option_separator);
    } else if option_datatype == "color" || option_datatype == "rgb" {
        let random_data = generator::color_generator(option_number, option_verbose);
        let _file_write_result = io::write_to_file(random_data, option_output_filename, option_separator);
    }

    print_in_verbose_mode(option_verbose, "\n ---> ", "white");
    print_in_verbose_mode(option_verbose, &String::from(option_number.to_string()).bold(), "green");

    if option_verbose {
        println!(
            "\n-----> {} {}",
            option_number.to_string().green().bold(),
            " data generated and wrote into file successfully.".green()
        );
    }

    Ok(())
}
