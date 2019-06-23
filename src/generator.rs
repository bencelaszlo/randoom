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
    T: std::string::ToString
{
    let mut generated_values = Vec::new();

    let mut rng = rand::thread_rng();
    for i in 0..n {
        let generated_value: T = rng.gen_range(lower_limit, higher_limit);
        if verbose_mode {
            print!(
                "\r--> {} / {}",
                (i + 1).to_string().cyan(),
                generated_value.to_string().blue()
            );
        }
        generated_values.push(generated_value);
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
