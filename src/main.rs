use std::fs::File;
use std::io::Write;

extern crate rand;
use rand::Rng;

extern crate clap;
use clap::{App, AppSettings, Arg};

extern crate colored;
use colored::*;

fn main() -> std::io::Result<()> {
    let app = App::new("randoom")
                        .setting(AppSettings::ColorAuto)
					    .setting(AppSettings::AllowNegativeNumbers)
                        .version("0.2.0")
                        .author("Bence László <bencelaszlo@protonmail.com>")
                        .about("Generate random numbers, texts, and colors.")

                        .arg(Arg::with_name("datatype")
                        .help("Choose the type of the generated data. Possible options: f32, f64, u32, u64, usize, i32, i64, isize, rgb.")
                        .short("t")
                        .takes_value(true)
                        .required(true) )

                        .arg(Arg::with_name("number")
                        .help("Number of data to be generated.")
                        .short("n")
                        .takes_value(true)
                        .required(true) )

                        .arg(Arg::with_name("lower_limit")
                        .help("Lower limit for numbers.")
                        .short("l")
                        .takes_value(true)
                        .required(false) )

                        .arg(Arg::with_name("higher_limit")
                        .help("Higher limit for numbers.")
                        .short("h")
                        .takes_value(true)
                        .required(false) )

                        .arg(Arg::with_name("separator")
                        .help("Separator character between individual values. Default value: \n (new line).")
                        .short("s")
                        .takes_value(true)
                        .required(false) )

                        .arg(Arg::with_name("output")
                        .help("Output filename. Default value: random_data.txt")
                        .short("o")
                        .takes_value(true)
                        .required(false) )

                        .get_matches();

    println!(
        "\n\t\t{}{}{}{}{}{}{}\n",
        "r".red().blink().underline().bold(),
        "a".yellow().blink().underline().bold(),
        "n".white().blink().underline().bold(),
        "d".cyan().blink().underline().bold(),
        "o".blue().blink().underline().bold(),
        "o".green().blink().underline().bold(),
        "m".magenta().blink().underline().bold()
    );
    println!("Already rewritten in Rust!");

    let mut option_datatype: String = "".to_string();
    let mut option_number: usize = 0;
    let mut option_lower_limit: f64 = 0.0f64;
    let mut option_higher_limit: f64 = 1.0f64;
    let mut option_separator: char = "\n".parse().unwrap();
    let mut option_output_filename: String = "./random_data.txt".to_string();

    println!(
        "{}",
        "Generate random data with the following parameters:".cyan()
    );

    if let Some(datatype) = app.value_of("datatype") {
        println!("choosen type: {}", datatype.cyan());
        option_datatype = datatype.to_string();
    }

    if let Some(number) = app.value_of("number") {
        println!("number: {}", number.cyan());
        option_number = number.parse().unwrap();
    }

    if let Some(lower_limit) = app.value_of("lower_limit") {
        println!("lower limit: {}", lower_limit.cyan());
        option_lower_limit = lower_limit.parse().unwrap();
    }

    if let Some(higher_limit) = app.value_of("higher_limit") {
        println!("higher_limit: {}", higher_limit.cyan());
        option_higher_limit = higher_limit.parse().unwrap();
    }

    if let Some(separator) = app.value_of("separator") {
        println!("separator: {}", separator.cyan());
        option_separator = separator.parse().unwrap();
    }

    if let Some(output) = app.value_of("output") {
        println!("output filename: {}", output.cyan());
        option_output_filename = output.parse().unwrap();
    }

    println!(
        "{}",
        "Generate data and write generated data into file...".magenta()
    );
    println!("{}", "Create file...".magenta());
    let mut buffer = File::create(option_output_filename)?;

    let mut rng = rand::thread_rng();

    println!("{}", "Generate random values...".magenta());

    if option_datatype == "f32" {
        for i in 0..option_number {
            let generated_data: f32 =
                rng.gen_range(option_lower_limit as f32, option_higher_limit as f32);
            print!(
                "\r--> {} / {}",
                (i + 1).to_string().cyan(),
                option_number.to_string().blue()
            );
            write!(buffer, "{}{}", generated_data, option_separator).unwrap();
        }
    } else if option_datatype == "f64" {
        for i in 0..option_number {
            let generated_data: f64 =
                rng.gen_range(option_lower_limit as f64, option_higher_limit as f64);
            print!(
                "\r--> {} / {}",
                (i + 1).to_string().cyan(),
                option_number.to_string().blue()
            );
            write!(buffer, "{}{}", generated_data, option_separator).unwrap();
        }
    } else if option_datatype == "u32" {
        for i in 0..option_number {
            let generated_data: u32 =
                rng.gen_range(option_lower_limit as u32, option_higher_limit as u32);
            print!(
                "\r--> {} / {}",
                (i + 1).to_string().cyan(),
                option_number.to_string().blue()
            );
            write!(buffer, "{}{}", generated_data, option_separator).unwrap();
        }
    } else if option_datatype == "u64" {
        for i in 0..option_number {
            let generated_data: u64 =
                rng.gen_range(option_lower_limit as u64, option_higher_limit as u64);
            print!(
                "\r--> {} / {}",
                (i + 1).to_string().cyan(),
                option_number.to_string().blue()
            );
            write!(buffer, "{}{}", generated_data, option_separator).unwrap();
        }
    } else if option_datatype == "usize" {
        for i in 0..option_number {
            let generated_data: usize =
                rng.gen_range(option_lower_limit as usize, option_higher_limit as usize);
            print!(
                "\r--> {} / {}",
                (i + 1).to_string().cyan(),
                option_number.to_string().blue()
            );
            write!(buffer, "{}{}", generated_data, option_separator).unwrap();
        }
    } else if option_datatype == "i32" {
        for i in 0..option_number {
            let generated_data: i32 =
                rng.gen_range(option_lower_limit as i32, option_higher_limit as i32);
            print!(
                "\r--> {} / {}",
                (i + 1).to_string().cyan(),
                option_number.to_string().blue()
            );
            write!(buffer, "{}{}", generated_data, option_separator).unwrap();
        }
    } else if option_datatype == "i64" {
        for i in 0..option_number {
            let generated_data: i64 =
                rng.gen_range(option_lower_limit as i64, option_higher_limit as i64);
            print!(
                "\r--> {} / {}",
                (i + 1).to_string().cyan(),
                option_number.to_string().blue()
            );
            write!(buffer, "{}{}", generated_data, option_separator).unwrap();
        }
    } else if option_datatype == "isize" {
        for i in 0..option_number {
            let generated_data: isize =
                rng.gen_range(option_lower_limit as isize, option_higher_limit as isize);
            print!(
                "\r--> {} / {}",
                (i + 1).to_string().cyan(),
                option_number.to_string().blue()
            );
            write!(buffer, "{}{}", generated_data, option_separator).unwrap();
        }
    /*} else if option_datatype == "char" {
    for i in 0..option_number {
        let generated_data: char = rng.gen_range(option_lower_limit, option_higher_limit).to_string().parse().unwrap();
        print!("\r{} / {}", (i + 1), option_number);
        write!(buffer, "{}{}", generated_data, option_separator).unwrap();
    }*/
    } else if option_datatype == "rgb" {
        for i in 0..option_number {
            let red_component_numeric: usize = rng.gen_range(0, 255);
            let green_component_numeric: usize = rng.gen_range(0, 255);
            let blue_component_numeric: usize = rng.gen_range(0, 255);

            let mut red_component: String = format!("{:x}", red_component_numeric);
            let mut green_component: String = format!("{:x}", green_component_numeric);
            let mut blue_component: String = format!("{:x}", blue_component_numeric);

            if red_component_numeric < 16 {
                red_component = red_component + "0";
            }

            if green_component_numeric < 16 {
                green_component = green_component + "0";
            }

            if blue_component_numeric < 16 {
                blue_component = blue_component + "0";
            }

            //            let red_component: String = format!("{:x}", rng.gen_range(0, 255) );
            //            let green_component: String = format!("{:x}", rng.gen_range(0, 255) );
            //            let blue_component: String = format!("{:x}", rng.gen_range(0, 255) );

            let generated_data = "#".to_string()
                + &red_component.to_string()
                + &green_component.to_string()
                + &blue_component.to_string();
            print!(
                "\r--> {} / {}",
                (i + 1).to_string().cyan(),
                option_number.to_string().blue()
            );
            write!(buffer, "{}{}", generated_data, option_separator).unwrap();
        }
    }

    println!(
        "\n-----> {} {}",
        option_number.to_string().green().bold(),
        " data generated and wrote into file successfully.".green()
    );

    Ok(())
}
