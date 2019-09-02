#!/bin/bash
cd /home/ubuntu/pastebin_scraper && git pull
. /home/ubuntu/pastebin_scraper/db.env
cargo build --release
./target/release/app