use std::{
    fs::File,
    io::Write,
};

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

pub fn write_to_file<T>(
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
