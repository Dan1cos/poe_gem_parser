use anyhow::Result;
use poe_gem_parser::*;
use pest::Parser;

#[test]
fn item_class_test() -> Result<()> {
    let parsed_data = GemParser::parse(Rule::item_class, "Item Class: Support Gems")?;
    assert_eq!(parsed_data.as_str(), "Support Gems");

    Ok(())
}

#[test]
fn class_test() -> Result<()> {
    let parsed_data = GemParser::parse(Rule::class, "Skill Gems")?;
    assert_eq!(parsed_data.as_str(), "Skill Gems");

    Ok(())
}

#[test]
fn rarity_test() -> Result<()> {
    let parsed_data = GemParser::parse(Rule::rarity, "Rarity: Gem")?;
    assert_eq!(parsed_data.as_str(), "Gem");

    Ok(())
}

#[test]
fn rarity_type_test() -> Result<()> {
    let parsed_data = GemParser::parse(Rule::rarity_type, "Gem")?;
    assert_eq!(parsed_data.as_str(), "Gem");

    Ok(())
}

#[test]
fn name_test() -> Result<()> {
    let parsed_data = GemParser::parse(Rule::name, "Enlighten Support")?;
    assert_eq!(parsed_data.as_str(), "Enlighten Support");

    Ok(())
}

#[test]
fn tags_test() -> Result<()> {
    let parsed_data = GemParser::parse(Rule::tags, "Attack, AoE")?;
    assert_eq!(parsed_data.as_str(), "Attack, AoE");

    Ok(())
}

#[test]
fn level_test() -> Result<()> {
    let parsed_data = GemParser::parse(Rule::level, "Level: 19")?;
    assert_eq!(parsed_data.as_str(), "19");

    Ok(())
}

#[test]
fn gem_level_test() -> Result<()> {
    let parsed_data = GemParser::parse(Rule::gem_level, "19")?;
    assert_eq!(parsed_data.as_str(), "19");

    Ok(())
}

#[test]
fn changes_test() -> Result<()> {
    let parsed_data = GemParser::parse(Rule::gem_changes, "Attack Speed: 80% of base")?;
    assert_eq!(parsed_data.as_str(), "Attack Speed: 80% of base");

    Ok(())
}

#[test]
fn quality_test() -> Result<()> {
    let parsed_data = GemParser::parse(Rule::quality, "Quality: +13% (augmented)")?;
    assert_eq!(parsed_data.as_str(), "13");

    Ok(())
}

#[test]
fn gem_quality_test() -> Result<()> {
    let parsed_data = GemParser::parse(Rule::gem_quality, "13")?;
    assert_eq!(parsed_data.as_str(), "13");

    Ok(())
}

#[test]
fn requirements_test() -> Result<()> {
    let parsed_data = GemParser::parse(Rule::requirements,  "Requirements:\nLevel: 12")?;
    assert_eq!(parsed_data.as_str(), "Level: 12");

    Ok(())
}

#[test]
fn requirement_test() -> Result<()> {
    let parsed_data = GemParser::parse(Rule::requirement,  "Int: 120")?;
    assert_eq!(parsed_data.as_str(), "Int: 120");

    Ok(())
}

#[test]
fn description_test() -> Result<()> {
    let parsed_data = GemParser::parse(Rule::description,  "Slam the ground, sending out rectangular fissures that deal area damage")?;
    assert_eq!(parsed_data.as_str(), "Slam the ground, sending out rectangular fissures that deal area damage");

    Ok(())
}

#[test]
fn modifiers_test() -> Result<()> {
    let parsed_data = GemParser::parse(Rule::modifiers,  "Base duration is 6.00 seconds\nShattering Spikes deal 30% less damage")?;
    assert_eq!(parsed_data.as_str(), "Base duration is 6.00 seconds\nShattering Spikes deal 30% less damage");

    Ok(())
}

#[test]
fn experience_test() -> Result<()> {
    let parsed_data = GemParser::parse(Rule::experience,  "Experience: 1/15249")?;
    assert_eq!(parsed_data.as_str(), "1/15249");

    Ok(())
}

#[test]
fn gem_experience_test() -> Result<()> {
    let parsed_data = GemParser::parse(Rule::gem_experience,  "1/15249")?;
    assert_eq!(parsed_data.as_str(), "1/15249");

    Ok(())
}

#[test]
fn usage_test() -> Result<()> {
    let parsed_data = GemParser::parse(Rule::usage,  "Place into an item socket of the right colour to gain this skill")?;
    assert_eq!(parsed_data.as_str(), "Place into an item socket of the right colour to gain this skill");

    Ok(())
}

#[test]
fn corrupted_test() -> Result<()> {
    let parsed_data = GemParser::parse(Rule::corrupted,  "Corrupted")?;
    assert_eq!(parsed_data.as_str(), "Corrupted");

    Ok(())
}

#[test]
fn note_test() -> Result<()> {
    let parsed_data = GemParser::parse(Rule::note,  "Note: buy this gem")?;
    assert_eq!(parsed_data.as_str(), "Note: buy this gem");

    Ok(())
}

#[test]
fn whitespace_test() -> Result<()> {
    let parsed_data = GemParser::parse(Rule::WHITESPACE, " ")?;
    assert_eq!(parsed_data.as_str(), "");

    Ok(())
}

#[test]
fn separator_test() -> Result<()> {
    let parsed_data = GemParser::parse(Rule::separator, "--------")?;
    assert_eq!(parsed_data.as_str(), "");

    Ok(())
}

#[test]
fn gem_test() -> Result<()> {
    let gem = "Item Class: Support Gems
    Rarity: Gem
    Swiftbrand Support
    --------
    Brand, Support
    Level: 21 (Max)
    Cost & Reservation Multiplier: 130%
    --------
    Requirements:
    Level: 72
    Int: 114
    --------
    Supports skills which create brands.
    --------
    Supported Skills have 65% less Attached Duration
    Supported Skills have 65% less Detached Duration
    Supported Skills have 45% more Activation frequency
    --------
    This is a Support Gem. It does not grant a bonus to your character, but to skills in sockets connected to it. Place into an item socket connected to a socket containing the Skill Gem you wish to augment. Right click to remove from a socket.
    --------
    Corrupted
    --------
    Note: ~b/o 1 chisel";

    let parsed_data = ParsedGem::parse(gem)?;
    assert_eq!(parsed_data.item_class, "Support Gems");
    assert_eq!(parsed_data.rarity, "Gem");
    assert_eq!(parsed_data.name, "Swiftbrand Support");
    assert_eq!(parsed_data.tags, ["Brand", "Support"]);
    assert_eq!(parsed_data.level, 21);
    assert_eq!(parsed_data.gem_changes, Some(vec!["Cost & Reservation Multiplier: 130%".to_string()]));
    assert_eq!(parsed_data.quality, 0);
    assert_eq!(parsed_data.requirements, ["Level: 72", "Int: 114"]);
    assert_eq!(parsed_data.description, "Supports skills which create brands.");
    assert_eq!(parsed_data.modifiers, Some(vec![ "Supported Skills have 65% less Attached Duration".to_string(), "Supported Skills have 65% less Detached Duration".to_string(), "Supported Skills have 45% more Activation frequency".to_string()]));
    assert_eq!(parsed_data.experience, None);
    assert_eq!(parsed_data.usage, "This is a Support Gem. It does not grant a bonus to your character, but to skills in sockets connected to it. Place into an item socket connected to a socket containing the Skill Gem you wish to augment. Right click to remove from a socket.");
    assert_eq!(parsed_data.corrupted, true);
    assert_eq!(parsed_data.note, Some("~b/o 1 chisel".to_string()));

    Ok(())
}


#[test]
#[should_panic]
fn error_test() {
    assert!(!GemParser::parse(Rule::name, "#wr0ng n4m3").is_err());
}