use anyhow::{anyhow, Result};
use clap::{arg, Parser};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Parser)]
struct ArgWord {
    /// The word to find meaning of
    #[arg(short, long)]
    word: String,
}

fn main() -> Result<()> {
    let args = ArgWord::parse();
    let meaning = get_meaning(&args.word)?;
    println!("{}", meaning);
    Ok(())
}

#[derive(Deserialize)]
struct UDDefinition {
    definition: String,
    #[serde(flatten)]
    _other: HashMap<String, serde_json::Value>,
}

#[derive(Deserialize)]
struct UDDefinitions {
    list: Vec<UDDefinition>,
}

fn get_meaning(query: &str) -> Result<String> {
    let url = format!("https://api.urbandictionary.com/v0/define?term={}", query);
    let json: String = ureq::get(&url)
        .call()
        .map_err(|e| anyhow!("Failed to fetch the URL: {:?}", e))?
        .into_string()
        .map_err(|e| anyhow!("Failed to parse the response: {:?}", e))?;

    let parsed_json: UDDefinitions =
        serde_json::from_str(&json).map_err(|e| anyhow!("Failed to parse the json: {:?}", e))?;
    // return the first definition found
    parsed_json
        .list
        .into_iter()
        .next()
        .map(|d| d.definition)
        .ok_or_else(|| anyhow!("Definition not found"))
}
