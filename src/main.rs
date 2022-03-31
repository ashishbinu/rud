use std::env;

use scraper::{Html, Selector};

fn main() {
    let search_query = env::args().nth(1).unwrap();
    let meaning = get_meaning(&search_query).unwrap();
    println!("{}", meaning);
}

fn get_meaning(query: &String) -> Result<String, ureq::Error> {
    let html: String =
        ureq::get(format!("https://www.urbandictionary.com/define.php?term={}", query).as_ref())
            .call()?
            .into_string()?;
    let parsed_html = Html::parse_document(&html);
    let selector = &Selector::parse("meta[name='Description']")
        .expect("Error during the parsing using the given selector");
    let meta_tag = parsed_html.select(&selector).nth(0).unwrap();
    let meaning = meta_tag.value().attr("content").unwrap().to_string();
    Ok(meaning)
}
