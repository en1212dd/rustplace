use std::env;
use std::io::ErrorKind;
use std::{error::Error, fs, fs::File, io};
use regex::Regex;
use text_colorizer::*;

#[derive(Debug)]
#[allow(dead_code)]
struct Arguments {
    pattern: String,
    replace: String,
    input_file: String,
    output_file: String,
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let args = env::args().skip(1).collect::<Vec<_>>();
    let text;
    if args.contains(&"--help".to_string()) || args.contains(&"-h".to_string()){
        print_help();
        return Ok(());
    }
    let arguments = validate_arguments(&args)?;
    if args.contains(&"-p".to_string()) {
        
        text = match search_and_remplace_in_regex(
            &arguments.pattern,
            &arguments.replace,
            &arguments.input_file,
        ) {
            Some(result) => result,
            None => {
                return Err("Not found any pattern in a file".into());
            }
        };
    }else{
        text = match search_and_remplace(
            &arguments.pattern,
            &arguments.replace,
            &arguments.input_file,
        ) {
            Some(result) => result,
            None => {
                return Err("Not found any pattern in a file".into());
            }
        };
    }
    create_or_remplace_file(&arguments.output_file, text)?;
    Ok(())
}

fn create_or_remplace_file(path: &str, text: String) -> io::Result<()> {
    let mut is_create = false;
    if let Err(e) = fs::write(path, &text) {
        if let ErrorKind::NotFound = e.kind() {
            File::create(path)?;
            is_create = true;
        }
    };
    if is_create {
        create_or_remplace_file(path, text)?;
    }
    Ok(())
}
fn search_and_remplace_in_regex(pattern: &str, replace: &str, input_file: &str) -> Option<String> {
    let reg = Regex::new(pattern).unwrap();
    let aux = input_file
        .lines()
        .into_iter()
        .map(|line| reg.replace_all(line, replace))
        .collect::<Vec<_>>();
    let compare_file = input_file.lines().into_iter().collect::<Vec<_>>();
    if aux == compare_file {
        return None;
    }
    let mut result = String::from("");
    for lines in aux {
        result += &(lines + "\n");
    }
    Some(result)
}
fn search_and_remplace(pattern: &str, replace: &str, input_file: &str) -> Option<String> {
    let aux = input_file
        .lines()
        .into_iter()
        .map(|line| line.replace(pattern, replace))
        .collect::<Vec<_>>();
    let compare_file = input_file.lines().into_iter().collect::<Vec<_>>();
    if aux == compare_file {
        return None;
    }
    let mut result = String::from("");
    for lines in aux {
        result += &(lines + "\n");
    }
    Some(result)
}
fn validate_arguments(args: &Vec<String>) -> Result<Arguments, Box<dyn Error>> {
    validation_number_args(args)?;
    validation_extension(args)?;
    println!("{:?}", args);
    let arguments;
    if args.contains(&"-p".to_string()){
        let input_file = fs::read_to_string(args[3].as_str())?;
        arguments = Arguments {
            pattern: args[1].to_string(),
            replace: args[2].to_string(),
            input_file,
            output_file: args[4].to_string(),
        };
    }else{
        let input_file = fs::read_to_string(args[2].as_str())?;
        arguments = Arguments {
            pattern: args[0].to_string(),
            replace: args[1].to_string(),
            input_file,
            output_file: args[3].to_string(),
        };
    }
    Ok(arguments)
}
fn validation_number_args(args: &Vec<String>) -> Result<(), String> {
    if args.len() < 4 {
        return Err(format!("{} wrong number of arguments given. Expected 4, got {} for more information see the {}",
        "\nError :".red().bold(), args.len(), "instructions: --help".yellow()
        ));
    }
    Ok(())
}
fn validation_extension(args: &Vec<String>) -> Result<(), String> {
    if args[0].trim().len() == 0 || args[1].trim().len() == 0 {
        return Err(format!("The length of the one of the parameters are 0 in this case are pattern:{} and replace_pattern:{}",
                args[0].len() , args[1].len()    
        ));
    }
    Ok(())
}

pub fn print_help() {
    eprintln!(
        "\t{} - repalce a string with a new string",
        "Find and Replace".green()
    );
    eprintln!("\tUsage:[use regex pattern] <target string> <replacemente string> <INTPUT FILE> <OUTPUT FILE>");
}
