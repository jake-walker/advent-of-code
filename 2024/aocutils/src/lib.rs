use std::{char, error::Error, fs};

pub fn read_input(name: &str) -> Result<String, Box<dyn Error>> {
    Ok(fs::read_to_string(format!("inputs/{}.txt", name))?
        .trim()
        .to_string())
}

pub fn read_input_lines(name: &str) -> Result<Vec<String>, Box<dyn Error>> {
    Ok(read_input(name)?
        .split("\n")
        .map(|v| v.to_string())
        .collect::<Vec<_>>())
}

pub fn read_input_grid(name: &str) -> Result<Vec<Vec<char>>, Box<dyn Error>> {
    Ok(read_input(name)?
        .split("\n")
        .map(|v| v.chars().collect())
        .collect::<Vec<Vec<char>>>())
}

pub fn read_input_lines_whitespace(name: &str) -> Result<Vec<Vec<String>>, Box<dyn Error>> {
    Ok(read_input(name)?
        .split("\n")
        .map(|v| {
            v.split_whitespace()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<Vec<_>>>())
}
