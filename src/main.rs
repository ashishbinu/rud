use anyhow::{anyhow, Result};
use clap::{ArgGroup, Parser};
use scraper::{Html, Selector};

#[derive(Parser)]
#[clap(version, about)]
#[clap(group(ArgGroup::new("wd").required(true).args(&["word"])))]
struct Args {
    /// The word to find meaning of
    word: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let meaning = get_meaning(&args.word)?;
    println!("{}", meaning);
    Ok(())
}

fn get_meaning(query: &str) -> Result<String> {
    let url = format!("https://www.urbandictionary.com/define.php?term={}", query);
    let html: String = ureq::get(&url)
        .call()
        .map_err(|e| anyhow!("Failed to fetch the URL: {:?}", e))?
        .into_string()
        .map_err(|e| anyhow!("Failed to parse the response: {:?}", e))?;

    let parsed_html = Html::parse_document(&html);
    let selector = Selector::parse("meta[name='Description']")
        .map_err(|e| anyhow!("Error during parsing using the given selector: {:?}", e))?;
    let meta_tag = parsed_html
        .select(&selector)
        .next()
        .ok_or_else(|| anyhow!("No meta tag found"))?;

    let meaning = meta_tag
        .value()
        .attr("content")
        .ok_or_else(|| anyhow!("No content attribute found"))?
        .to_string();
    Ok(meaning)
}
