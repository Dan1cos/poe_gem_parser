# PoE Gem Parser in Rust

A parser for normal gems from game "Path of Exile" written in Rust. That project was developed for educational purposes, with the main aim of enhancing skills in creating projects using Rust and PEST for building custom parser. That project can be further extended for parsing not only gems, but also transfigured/vaal gems, items, fragments, etc. 

## Developed grammar rules
gem - final grammar rule for parsing normal gem  
item_class - for parsing item_class of item  
class - possible classes of item  
rarity - rarity of item  
name - name of item  
separator - separators between blocks  




## Dependencies
Dependencies used in `Cargo.toml`:

```toml
pest_derive = "2.7.14"
pest = "2.7.14"
anyhow = "1.0.91"
thiserror = "2.0.3"