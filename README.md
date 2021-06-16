# Brilliant.org community scraper
Used to scrape the Community problems page for all its' content.

Please ensure the ".contributions.cache" file is in your working directory so you don't have to re-scrape all the (28,272) contributions' urls again. It takes ~1 hour.

Side Note: Unknown if this is all of them. But this is where mine ended.

# Starting

## To run scraper (use cmd/terminal)
```bash
./scrape-brilliant scrape
```

## To run web server (use cmd/terminal)
```bash
./scrape-brilliant web
```

The webserver is used for viewing what you scraped easily.


# Compiling

If you'd like to compile and run it instead of using the pre-made exe you can easily.

Installing Rust is super simple.
```
https://www.rust-lang.org/tools/install
```

After its' installed. Run the command in the main directory (it contains src, Cargo.toml, etc..):
```bash
cargo build
```

After its' built you can run it with these two commands.
```bash
cargo run -- web
```
OR
```bash
cargo run -- scrape
```