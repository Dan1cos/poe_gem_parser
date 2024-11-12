use anyhow::Result;
use poe_gem_parser::*;
use pest::Parser;

fn main() -> Result<()> {
    let gem_text = "Item Class: Support Gems
    Rarity: Gem
    Less Duration Support
    --------";
    let parsed = Grammar::parse(Rule::gem, gem_text)?;

    println!("{:#?}", parsed);

    Ok(())
}
