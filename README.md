# PoE Gem Parser in Rust

A parser for normal gems from game "Path of Exile" written in Rust. That project was developed for educational purposes, with the main aim of enhancing skills in creating projects using Rust and PEST for building custom parser. That project can be further extended for parsing not only gems, but also transfigured/vaal gems, items, fragments, etc. 

## Developed grammar rules
item_class - for parsing class of item 
rarity - rarity of item  
name - name of item  
tags - tags of item  
level - level of gem  
gem_changes - basic stats that are changed using this gem  
quality - quality of gem  
requirements - stats that are rquired for equipping this gem  
description - description of the gem  
usage - how that gem should be used in a game  
corrupted - is item corrupted or not  
note - note  
separator - separators between blocks  


## Dependencies
Dependencies used in `Cargo.toml`:

```toml
pest_derive = "2.7.14"
pest = "2.7.14"
anyhow = "1.0.91"
thiserror = "2.0.3"