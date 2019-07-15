extern crate rand;
use rand::Rng;

use colored::Colorize;

extern crate num_traits;

pub fn number_generator<T: num_traits::Num>(
    n: usize,
    lower_limit: T,
    higher_limit: T,
    verbose_mode: bool,
) -> Vec<T>
where
    T: rand::distributions::uniform::SampleUniform,
    T: std::marker::Copy,
    T: std::string::ToString,
{
    let mut generated_values = Vec::new();

    let mut rng = rand::thread_rng();
    for i in 0..n {
        let generated_value: T = rng.gen_range(lower_limit, higher_limit);
        if verbose_mode {
            print!(
                "\r--> {} / {}",
                (i + 1).to_string().cyan(),
                n.to_string().blue()
            );
        }
        generated_values.push(generated_value);
    }

    return generated_values;
}

pub fn bool_generator(n: usize, verbose_mode: bool) -> Vec<bool> {
    let mut generated_values = Vec::new();

    let mut rng = rand::thread_rng();
    for i in 0..n {
        let temp_num_value = rng.gen_range(0, 2);
        let bool_value: bool;
        if temp_num_value == 0 {
            bool_value = false;
        } else {
            bool_value = true;
        }

        if verbose_mode {
            print!(
                "\r--> {} / {}",
                (i + 1).to_string().cyan(),
                n.to_string().blue()
            );
        }
        generated_values.push(bool_value);
    }

    return generated_values;
}

fn is_control_char(c: u8) -> bool {
    let mut is_control: bool = false;
    if c <= 32 || (c >= 127 && c <= 160) {
        is_control = true;
    }
    return is_control;
}

fn is_numeric_char(c: u8) -> bool {
    let mut is_numeric: bool = false;
    if c >= 48 && c <= 57 {
        is_numeric = true;
    }
    return is_numeric;
}

fn is_letter_char(c: u8) -> bool {
    let mut is_letter: bool = false;
    if (c >= 65 && c <= 90)
        || (c >= 97 && c <= 122)
        || (c >= 192 && c <= 214)
        || (c >= 216 && c <= 246)
        || c >= 248
    {
        is_letter = true;
    }
    return is_letter;
}

fn is_spec_char(c: u8) -> bool {
    let mut is_spec: bool = false;
    if (c >= 33 && c <= 47)
        || (c >= 58 && c <= 64)
        || (c >= 91 && c <= 96)
        || (c >= 123 && c <= 126)
        || (c >= 161 && c <= 191)
        || c == 215
        || c == 247
    {
        is_spec = true;
    }
    return is_spec;
}

pub fn char_generator(
    n: usize,
    special_characters_mode: bool,
    numbers_mode: bool,
    verbose_mode: bool,
) -> Vec<char> {
    let mut generated_values = Vec::new();

    let mut rng = rand::thread_rng();

    while generated_values.len() != n {
        let temp_data: &[u8] = &[rng.gen_range(0, 255) as u8];

        if numbers_mode && special_characters_mode {
            if !is_control_char(temp_data[0]) {
                let generated_char: char = temp_data[0] as char;
                generated_values.push(generated_char);
                if verbose_mode {
                    print!(
                        "\r--> {} / {}",
                        generated_values.len().to_string().cyan(),
                        n.to_string().blue()
                    );
                }
            }
        } else if !numbers_mode && special_characters_mode {
            if !is_control_char(temp_data[0]) && !is_numeric_char(temp_data[0]) {
                let generated_char: char = temp_data[0] as char;
                generated_values.push(generated_char);
                if verbose_mode {
                    print!(
                        "\r--> {} / {}",
                        generated_values.len().to_string().cyan(),
                        n.to_string().blue()
                    );
                }
            }
        } else if numbers_mode && !special_characters_mode {
            if !is_control_char(temp_data[0]) && !is_spec_char(temp_data[0]) {
                let generated_char: char = temp_data[0] as char;
                generated_values.push(generated_char);
                if verbose_mode {
                    print!(
                        "\r--> {} / {}",
                        generated_values.len().to_string().cyan(),
                        n.to_string().blue()
                    );
                }
            }
        } else if !numbers_mode && !special_characters_mode {
            if !is_control_char(temp_data[0])
                && !is_numeric_char(temp_data[0])
                && !is_spec_char(temp_data[0])
            {
                let generated_char: char = temp_data[0] as char;
                generated_values.push(generated_char);
                if verbose_mode {
                    print!(
                        "\r--> {} / {}",
                        generated_values.len().to_string().cyan(),
                        n.to_string().blue()
                    );
                }
            }
        }
    }

    return generated_values;
}

pub fn color_generator(n: usize, verbose_mode: bool) -> Vec<String> {
    let mut generated_values = Vec::new();

    let mut rng = rand::thread_rng();
    for i in 0..n {
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

        let generated_value = "#".to_string()
            + &red_component.to_string()
            + &green_component.to_string()
            + &blue_component.to_string();

        if verbose_mode {
            print!(
                "\r--> {} / {}",
                (i + 1).to_string().cyan(),
                n.to_string().blue()
            );
        }
        generated_values.push(generated_value);
    }

    return generated_values;
}
