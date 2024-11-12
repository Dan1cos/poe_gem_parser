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
fn tags_test() -> Result<()> {
    let parsed_data = Grammar::parse(Rule::tags, "Attack, AoE")?;
    assert_eq!(parsed_data.as_str(), "Attack, AoE");

    Ok(())
}

#[test]
fn level_test() -> Result<()> {
    let parsed_data = Grammar::parse(Rule::level, "Level: 19")?;
    assert_eq!(parsed_data.as_str(), "Level: 19");

    Ok(())
}

#[test]
fn changes_test() -> Result<()> {
    let parsed_data = Grammar::parse(Rule::gem_changes, "Attack Speed: 80% of base")?;
    assert_eq!(parsed_data.as_str(), "Attack Speed: 80% of base");

    Ok(())
}

#[test]
fn quality_test() -> Result<()> {
    let parsed_data = Grammar::parse(Rule::quality, "Quality: +13% (augmented)")?;
    assert_eq!(parsed_data.as_str(), "Quality: +13% (augmented)");

    Ok(())
}

#[test]
fn requirements_test() -> Result<()> {
    let parsed_data = Grammar::parse(Rule::requirements,  "Requirements:\nLevel: 12")?;
    assert_eq!(parsed_data.as_str(), "Requirements:\nLevel: 12");

    Ok(())
}

#[test]
fn requirement_test() -> Result<()> {
    let parsed_data = Grammar::parse(Rule::requirement,  "Int: 120")?;
    assert_eq!(parsed_data.as_str(), "Int: 120");

    Ok(())
}

#[test]
fn description_test() -> Result<()> {
    let parsed_data = Grammar::parse(Rule::description,  "Slam the ground, sending out rectangular fissures that deal area damage")?;
    assert_eq!(parsed_data.as_str(), "Slam the ground, sending out rectangular fissures that deal area damage");

    Ok(())
}

#[test]
fn modifiers_test() -> Result<()> {
    let parsed_data = Grammar::parse(Rule::modifiers,  "Base duration is 6.00 seconds\nShattering Spikes deal 30% less damage")?;
    assert_eq!(parsed_data.as_str(), "Base duration is 6.00 seconds\nShattering Spikes deal 30% less damage");

    Ok(())
}

#[test]
fn experience_test() -> Result<()> {
    let parsed_data = Grammar::parse(Rule::experience,  "Experience: 1/15249")?;
    assert_eq!(parsed_data.as_str(), "Experience: 1/15249");

    Ok(())
}

#[test]
fn usage_test() -> Result<()> {
    let parsed_data = Grammar::parse(Rule::usage,  "Place into an item socket of the right colour to gain this skill")?;
    assert_eq!(parsed_data.as_str(), "Place into an item socket of the right colour to gain this skill");

    Ok(())
}

#[test]
fn corrupted_test() -> Result<()> {
    let parsed_data = Grammar::parse(Rule::corrupted,  "Corrupted")?;
    assert_eq!(parsed_data.as_str(), "Corrupted");

    Ok(())
}

#[test]
fn note_test() -> Result<()> {
    let parsed_data = Grammar::parse(Rule::note,  "Note: buy this gem")?;
    assert_eq!(parsed_data.as_str(), "Note: buy this gem");

    Ok(())
}

#[test]
#[should_panic]
fn error_test() {
    assert!(!Grammar::parse(Rule::name, "#wr0ng n4m3").is_err());
}