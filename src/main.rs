use std::io::Write;
use std::fs::File;

extern crate rand;
use rand::Rng;

//extern crate clap;
//use clap::{Arg, App, SubCommand};

fn main() -> std::io::Result<()> {
//    let matches = App::new("RanDoom")
//                        .version("0.1.0")
//                        .author("Bence László <bencelaszlo@protonmail.com>")
//                        .about("Generate random numbers, texts, and colors.")
//                        .arg(Arg::with_name("config"))

    println!("Generate random numbers");
    let number_of_numbers: usize = 100000;
    let mut numbers = Vec::new();

    let mut rng = rand::thread_rng();
    for i in 0..number_of_numbers {
        let temp_number: f64 = rng.gen_range(-250.0f64, 250.0f64);
        numbers.push(temp_number);
    }

    let separator = ", "; //"\n", " " should be options, too.

    println!("WRITE GENERATED DATA INTO FILE");
    println!("Create file...");
    let mut buffer = File::create("random_data.txt")?;

    print!("Write data...");
    for i in 0..(number_of_numbers-1) {
        print!("\rWriting: {} / {}", i+1, number_of_numbers);
        write!(buffer, "{}{}", numbers[i], separator).unwrap();
    }
    println!("\rWriting: {} / {}", number_of_numbers, number_of_numbers);
    write!(buffer, "{}", numbers[number_of_numbers-1]).unwrap();

    println!("{} data generated and wrote into file successfully.", number_of_numbers);

    Ok(())
}
