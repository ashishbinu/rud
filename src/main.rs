use clap::{ArgGroup, Parser};

use scraper::{Html, Selector};

#[derive(Parser)]
#[clap(version, about)]
#[clap(group(
            ArgGroup::new("wd")
                .required(true)
                .args(&["word"]),
        ))]
struct Args {
    /// The word to find meaning of
    word: String,
}

fn main() {
    let args = Args::parse();
    let meaning = get_meaning(&args.word).unwrap();
    println!("{}", meaning);
}

fn get_meaning(query: &str) -> Result<String, ureq::Error> {
    let html: String =
        ureq::get(format!("https://www.urbandictionary.com/define.php?term={}", query).as_ref())
            .call()?
            .into_string()?;
    let parsed_html = Html::parse_document(&html);
    let selector = &Selector::parse("meta[name='Description']")
        .expect("Error during the parsing using the given selector");
    let meta_tag = parsed_html.select(selector).next().unwrap();
    let meaning = meta_tag.value().attr("content").unwrap().to_string();
    Ok(meaning)
}
