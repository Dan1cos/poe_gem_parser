use pest::Parser;
use pest_derive::Parser;
use thiserror::Error;

#[derive(Parser)]
#[grammar = "./grammar.pest"]
pub struct GemParser;

#[derive(Error, Debug)]
pub enum GemParseError {
    #[error("failed to parse gem: {0}")]
    ParseError(String),
}

#[derive(Debug)]
pub struct ParsedGem {
    pub item_class: String, 
    pub rarity: String,
    pub name: String,
    pub tags: Vec<String>,
    pub level: i32,
    pub gem_changes: Option<Vec<String>>,
    pub quality: i32,
    pub requirements: Vec<String>,
    pub description: String,
    pub modifiers: Option<Vec<String>>,
    pub experience: Option<String>,
    pub usage: String,
    pub corrupted: bool,
    pub note: Option<String>
}

impl ParsedGem {
    pub fn parse(input: &str) -> Result<Self, GemParseError> {
        let parsed_pairs = GemParser::parse(Rule::gem, input).map_err(|e| GemParseError::ParseError(e.to_string()))?;

        let mut item_class = String::new();
        let mut rarity = String::new();
        let mut name = String::new();
        let mut tags = Vec::new();
        let mut level = 0;
        let mut gem_changes = None;
        let mut quality = 0;
        let mut requirements = Vec::new();
        let mut description = String::new();
        let mut modifiers = None;
        let mut experience = None;
        let mut usage = String::new();
        let mut corrupted = false;
        let mut note = None; 

        for pair in parsed_pairs.into_iter() {
            match pair.as_rule() {
                Rule::class => item_class = pair.as_str().trim().to_string(),
                Rule::rarity_type => rarity = pair.as_str().trim().to_string(),
                Rule::name => name = pair.as_str().trim().to_string(),
                Rule::tags => tags.extend(pair.as_str().split(",").map(|i| i.trim().to_string())),
                Rule::gem_level => level = pair.as_str().trim().parse().unwrap(),
                Rule::gem_changes => gem_changes = Some(pair.as_str().trim().split("\n").map(|i| i.trim().to_string()).collect()),
                Rule::gem_quality => quality = pair.as_str().trim().parse().unwrap(),
                Rule::requirement => requirements.push(pair.as_str().trim().to_string()),
                Rule::description => description = pair.as_str().trim().to_string(),
                Rule::modifiers => modifiers = Some(pair.as_str().trim().split("\n").map(|i| i.trim().to_string()).collect()),
                Rule::gem_experience => experience = Some(pair.as_str().trim().to_string()),
                Rule::usage => usage = pair.as_str().trim().to_string(),
                Rule::corrupted => corrupted = true,
                Rule::note => note = Some(pair.as_str().trim().to_string()),
                _ => {}
            }
        }

        Ok(ParsedGem {
            item_class, 
            rarity,
            name,
            tags,
            level,
            gem_changes,
            quality,
            requirements,
            description,
            modifiers,
            experience,
            usage,
            corrupted,
            note
        })
    }
}