use anyhow::Result;
use poe_gem_parser::*;
use std::env;

fn help() {
    println!("PoE Gem Parser -- Help");
    println!("cargo run <GemText>        Parse gem text");
    println!("cargo run -- --help        Help");
    println!("cargo run -- --credits     Credits");
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
        gem => println!("{:#?}", ParsedGem::parse(gem)?)
    }

    Ok(())
}
