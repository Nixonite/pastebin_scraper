extern crate reqwest;
extern crate scraper;
extern crate postgres;

// importation syntax
use scraper::{Html, Selector};
use std::time::{Duration, SystemTime};
use std::thread::sleep;
use postgres::{Connection, TlsMode, Error};
use std::env;

fn main() {
  let conn = Connection::connect(
    format!("postgresql://{}:{}@{}:{}/{}",
      env::var("postgres_user").unwrap(),
      env::var("postgres_password").unwrap(),
      env::var("postgres_host").unwrap(),
      env::var("postgres_port").unwrap(),
      env::var("postgres_db").unwrap()),
    TlsMode::None).unwrap();

  conn.execute("CREATE TABLE IF NOT EXISTS pastes (
                    id              SERIAL PRIMARY KEY,
                    paste_id        TEXT,
                    paste_key       TEXT,
                    dtstamp         TIMESTAMP,
                      UNIQUE(paste_id)
                  )", &[]).unwrap();


  let mut resp = reqwest::get("https://pastebin.com/archive").unwrap();
  assert!(resp.status().is_success());

  let body = resp.text().unwrap();
  // parses string of HTML as a document
  let fragment = Html::parse_document(&body);
  // parses based on a CSS selector
  let table_selector = Selector::parse(".maintable").unwrap();
  let link_selector = Selector::parse("a").unwrap();

  let table = fragment.select(&table_selector).next().unwrap();
  let paste_urls = table
    .select(&link_selector)
    .filter_map(|e| e.value().attr("href"))
    .filter(|s| s[1..].len() == 8);

  for url in paste_urls {
    let paste_url = format!("https://pastebin.com/raw{}", url);
    let paste_text = reqwest::get(&paste_url).unwrap().text().unwrap();

    conn.execute("INSERT INTO pastes (paste_id, paste_key, dtstamp) VALUES ($1, $2, now()) ON CONFLICT DO NOTHING",
      &[&paste_url, &paste_text]).unwrap();

    sleep(Duration::new(1, 0));
  }
}