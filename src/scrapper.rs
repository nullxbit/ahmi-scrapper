use anyhow::{Ok, Result};
#[allow(unused_imports)]
use reqwest::{self, blocking::get};
use scraper::{Html, Selector};
// use std::io::{self, Write};

pub fn scrapper(input: String) -> Result<()> {
    let baseurl = format!("https://ahmia.fi/search/?q={input}&7db214=2c583a");

    let req = reqwest::blocking::get(baseurl)?.text()?;
    // println!("{:?}", req);
    let fragment = Html::parse_document(req.as_str());
    let selector = Selector::parse("li cite").unwrap();
    for elements in fragment.select(&selector) {
        let url = elements.text().collect::<String>().trim().to_string();
        println!("{}", url);
    }

    Ok(())
}

