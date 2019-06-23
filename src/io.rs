use std::{
    fs::File,
    io::{BufRead, BufReader, Write},
};

use serde_json::Value;

pub fn write_numbers_to_file<T: num_traits::Num>(
    vector_to_write: Vec<T>,
    output_filename: String,
    separator_char: char,
) -> std::io::Result<()>
where
    T: std::fmt::Display,
{
    let mut buffer = File::create(output_filename)?;

    for line in vector_to_write {
        write!(buffer, "{}{}", line, separator_char).unwrap();
    }

    Ok(())
}

pub fn write_to_file<T>(vector_to_write: Vec<T>, output_filename: String, separator_char: char) -> std::io::Result<()>
where
    T: std::fmt::Display
 {
    let mut buffer = File::create(output_filename)?;

    for line in vector_to_write {
        write!(buffer, "{}{}", line, separator_char).unwrap();
    }

    Ok(())
}

fn read_lines_from_file(file_path: String, json_string: &mut String) -> std::io::Result<()> {
    let f = File::open(file_path)?;
    let f = BufReader::new(f);

    for line in f.lines() {
        let line = line?;
        let mut processed_line: String = line.split_whitespace().collect(); // Remove whitespaces from the string
        json_string.push_str(&processed_line);
    }

    Ok(())
}

pub fn deserialize_json_schema(file_path: String) -> std::io::Result<()> {
    let mut json_string = "".to_string();
    let _result = read_lines_from_file(file_path, &mut json_string);
    println!("json_string={}", json_string);

    let json_value: Value = serde_json::from_str(&json_string)?;
    println!("{}", json_value);

    for (key, value) in json_value.as_object().unwrap() {
        println!("{:?} ===> {:?}", key, value);
    }

    Ok(())
}
