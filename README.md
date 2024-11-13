# PoE Gem Parser in Rust

A parser for normal gems from game "Path of Exile" written in Rust. Parser is written using `pest` library and can extract various fields(name, tags, level, requirements, etc.) of the selected gem. That project can be further extended for parsing not only gems, but also transfigured/vaal gems, items, fragments, etc. (Gem text for testing can be taken from Path of Exile's trade site)


## Description

Parser reads input string from arguments or file and splits it into components using predifined `pest` grammar rule, that components are later combined in a `ParsedGem` structure.


## Developed grammar rules

WHITESPACE - any whitespace characters  
separator - separator between blocks  
gem - complete gem grammar rule  
item_class - class of item with "Item class:" string  
class - class of item  
rarity - rarity type of item with "Rarity:" string  
rarity_type - rarity type  
name - name of item  
tags - tags of item  
level - level of gem with "Level:" and "(Max)" strings  
gem_level - level of gem  
gem_changes - stats that are changed using this gem  
quality - quality of gem with "Quality:" and "(augmented)" strings  
gem_quality - quality of gem  
requirements - requirements this gem with "Requirements:" string  
requirement - one of requirements  
description - description of the gem  
modifiers - modifiers of gem  
usage - how that gem should be used in a game  
corrupted - is item corrupted or not  
note - note  

## Example 

```
Item Class: Support Gems     --------- class = "Support Gems"
Rarity: Gem                  --------- rarity = "Gem"
Enlighten Support            --------- name = "Enlighten Support"
--------
Exceptional, Support         --------- tags = ["Exceptional", "Support"]
Level: 2                     --------- level = 2
Reservation Multiplier: 96%  --------- gem_changes = ["Reservation Multiplier: 96%"]
Quality: +9% (augmented)     --------- quality = 9
--------
Requirements:                --------- requirements = ["Level: 1"]
Level: 1                 
--------
Supports any skill gem       --------- description = "Supports any skill gem"
--------
Gains increased Experience   --------- modifiers = ["Gains increased Experience"]
--------
Experience: 1/226854909      --------- experience = "1/226854909"
--------
This is a Support Gem        --------- usage = "This is a Support Gem"
--------
Corrupted                    --------- corrupted = true
--------
Note: ~price 30 chaos        --------- note = "~price 30 chaos"
```


## Dependencies
Dependencies used in `Cargo.toml`:

```toml
pest_derive = "2.7.14"
pest = "2.7.14"
anyhow = "1.0.91"
thiserror = "2.0.3"
```