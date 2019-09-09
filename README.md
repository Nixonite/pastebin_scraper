# Pastebin Scraper

This Rust program is run every few minutes to scrape data not already existing in my database.

Just cargo run it.

Fetches from the archive a list of paste urls/ids, then goes one by one every 4 seconds to scrape them. 
