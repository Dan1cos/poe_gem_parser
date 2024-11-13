use anyhow::Result;
use poe_gem_parser::*;
use pest::Parser;

fn main() -> Result<()> {
    let gem_text = "Item Class: Support Gems
    Rarity: Gem
    Trap Support
    --------
    Support, Trap
    Level: 1
    Cost & Reservation Multiplier: 120%
    Quality: +9% (augmented)
    --------
    Requirements:
    Level: 8
    Int: 20
    --------
    Supports spells, or attacks that use bows or wands. Instead of using that skill, you will throw a trap that will use the skill for you when an enemy walks near it. Traps cannot use channelling skills.
    --------
    Supported Attack Skills cannot be used with Melee Weapons
    Trap lasts 4 seconds
    Supported Skills deal 20% less Trap Damage
    Supported Skills deal 4% increased Trap Damage
    --------
    Experience: 1/3231
    --------
    This is a Support Gem. It does not grant a bonus to your character, but to skills in sockets connected to it. Place into an item socket connected to a socket containing the Skill Gem you wish to augment. Right click to remove from a socket.
    --------
    Corrupted";

    let gem_text_corrupted_max = "Item Class: Skill Gems
    Rarity: Gem
    Stormblast Mine
    --------
    Mine, Spell, AoE, Lightning, Aura, Nova
    Level: 20 (Max)
    Cost: 6 Mana
    Reservation: 6 Mana
    Cast Time: 0.75 sec
    Critical Strike Chance: 6.00%
    Effectiveness of Added Damage: 130%
    --------
    Requirements:
    Level: 72
    Dex: 70
    Int: 100
    --------
    Throws a mine that deals damage in an area when detonated.
    --------
    Deals 410 to 1231 Lightning Damage
    Mine lasts 5 seconds
    Base Mine Detonation Time is 0.25 seconds
    +0.6 metres to radius
    Each Mine applies 3% increased Damage Taken to Enemies near it, up
    to a maximum of 150%
    --------
    Place into an item socket of the right colour to gain this skill. Right click to remove from a socket.
    --------
    Corrupted
    --------
    Note: ~price 1 chaos";

    
    // let parsed = GemParser::parse(Rule::gem, gem_text)?;

    let parsed = ParsedGem::parse(gem_text);

    println!("{:#?}", parsed);

    Ok(())
}
