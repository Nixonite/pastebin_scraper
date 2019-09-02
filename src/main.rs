extern crate reqwest;
extern crate scraper;
extern crate postgres;

// importation syntax
use scraper::{Html, Selector};
use std::time::{Duration, SystemTime};
use std::thread::sleep;
use postgres::{Connection, TlsMode, Error};
use std::env;
use std::vec;

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

  let body = resp.text().unwrap();
  let fragment = Html::parse_document(&body);
  let table_selector = Selector::parse(".maintable").unwrap();
  let link_selector = Selector::parse("a").unwrap();

  let table = fragment.select(&table_selector).next().unwrap();
  let paste_urls = table
    .select(&link_selector)
    .filter_map(|e| e.value().attr("href"))
    .filter(|s| s[1..].len() == 8);

  let precheck_urls_query = format!(
    "with temp(paste_id) as (values {})
    select temp.paste_id from temp
    left join pastes
      on temp.paste_id = pastes.paste_id
    where pastes.paste_id is null",
    paste_urls
      .map(|s| format!("(\'https://pastebin.com/raw{}\')", s))
      .collect::<Vec<_>>()
      .join(", ")
    );

  for url in &conn.query(&precheck_urls_query, &[]).unwrap() {
    let uri: String = url.get(0);
    let paste_url = format!("https://pastebin.com/raw{}", uri);
    let paste_text = reqwest::get(&paste_url).unwrap().text().unwrap();

    conn.execute("
      INSERT INTO 
      pastes (paste_id, paste_key, dtstamp)
      VALUES ($1, $2, now())
      ON CONFLICT DO NOTHING",
      &[&paste_url, &paste_text]).unwrap();

    sleep(Duration::new(1, 0));
  }
}