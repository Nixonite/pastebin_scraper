#!/bin/bash

. /home/ubuntu/pastebin_scraper/db.env
cargo build --release
./target/release/app