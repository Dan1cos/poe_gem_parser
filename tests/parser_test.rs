use anyhow::Result;
use poe_gem_parser::*;
use pest::Parser;

#[test]
fn item_class_test() -> Result<()> {
    let parsed_data = Grammar::parse(Rule::item_class, "Item Class: Support Gems")?;
    assert_eq!(parsed_data.as_str(), "Item Class: Support Gems");

    Ok(())
}

#[test]
fn class_test() -> Result<()> {
    let parsed_data = Grammar::parse(Rule::class, "Skill Gems")?;
    assert_eq!(parsed_data.as_str(), "Skill Gems");

    Ok(())
}

#[test]
fn rarity_test() -> Result<()> {
    let parsed_data = Grammar::parse(Rule::rarity, "Rarity: Gem")?;
    assert_eq!(parsed_data.as_str(), "Rarity: Gem");

    Ok(())
}

#[test]
fn name_test() -> Result<()> {
    let parsed_data = Grammar::parse(Rule::name, "Enlighten Support")?;
    assert_eq!(parsed_data.as_str(), "Enlighten Support");

    Ok(())
}

#[test]
#[should_panic]
fn error_test() {
    assert!(!Grammar::parse(Rule::name, "#wr0ng n4m3").is_err());
}