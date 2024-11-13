use anyhow::Result;
use poe_gem_parser::*;
use std::env;
use std::fs;

fn help() {
    println!("PoE Gem Parser -- Help");
    println!("cargo run <GemText>                               Parse gem text");
    println!("cargo run -- --file <iunput_file> <output_file>   Parse gem from txt file");
    println!("cargo run -- --help                               Help");
    println!("cargo run -- --credits                            Credits");
}

fn credits() {
    println!("PoE Gem Parser -- Credits");
    println!("PoE Gem Parser by Artem Shakirov");
}


fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Wrong number of arguments, --help displayed");
        help();
        return Ok(());
    }

    match args[1].as_str() {
        "--help" => help(),
        "--credits" => credits(),
        "--file" => {
            if args.len() < 4 {
                println!("Wrong number of arguments for parsing file");
                return Ok(());
            } 
            let input_file = args[2].as_str();
            let output_file = args[3].as_str();
            let input = fs::read_to_string(input_file)?;
            let parsed = ParsedGem::parse(input.as_str())?;
            fs::write(output_file, format!("{:#?}", parsed))?;
        },
        gem => println!("{:#?}", ParsedGem::parse(gem)?)
    }

    Ok(())
}
