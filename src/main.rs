#[macro_use]
extern crate prettytable;
extern crate reqwest;
extern crate select;

use prettytable::Table;
use select::document::Document;
use select::predicate::{Name};
use std::collections::HashMap;

fn main() {
    let query = std::env::args().nth(1);
    cricket_score_list("http://static.cricinfo.com/rss/livescores.xml", query);
}

fn cricket_score_list(url: &str, query: Option<String>) {
    let resp = reqwest::get(url).unwrap();
    assert!(resp.status().is_success());

    let document = Document::from_read(resp).unwrap();
    let mut matches: Vec<HashMap<String, String>> = Vec::new();
    for match_node in document.find(Name("item")) {
        let mut match_map = HashMap::new();
        let score = match_node.find(Name("title")).next().unwrap();
        let link = match_node.find(Name("guid")).next().unwrap();
        match_map.insert("link".to_string(), link.text().trim().to_string());
        match_map.insert("title".to_string(), score.text().trim().to_string());
        matches.push(match_map);
    }
    match query {
        None => print_matches(matches),
        Some(search_query) => {
            let found_match = matches
                .iter()
                .find(|&m| m["title"].to_ascii_lowercase().contains(&search_query));
            match found_match {
                None => {
                    println!(
                        "Could find a match with query '{}'. Here is the list of live matches:\n",
                        search_query
                    );
                    print_matches(matches);
                }
                Some(m) => {
                    live_cricket_score(m["link"].trim());
                }
            }
        }
    }
}

fn print_matches(matches: Vec<HashMap<String, String>>) {
    let mut table = Table::new();
    for (i, match_map) in matches.iter().enumerate() {
        table.add_row(row![Fyb-> i + 1, match_map["title"]]);
    }
    table.printstd();
}

fn live_cricket_score(url: &str) {
    let resp = reqwest::get(url).unwrap();
    assert!(resp.status().is_success());

    let document = Document::from_read(resp).unwrap();
    let page_title = document.find(Name("title")).next().unwrap();
    let title_text = page_title.text();
    println!("{}", title_text)
}
