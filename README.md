# Brilliant.org community scraper
**Answer scraping is not implemented. Only all the problems.**

Used to scrape the Community problems page for all its' content. Will currently scrape the "New", "Popular", "Discussions", and "Needs Solution" Contributions

Please ensure the ".contributions.cache" file is in your working directory so you don't have to re-scrape all the (121,258) contributions' urls again. It takes ~3 hour.

Side Note: Unknown if this is all of them. But this is where mine ended.


# Starting

## To run scraper (use cmd/terminal)
```bash
./scrape-brilliant scrape
or
./scrape-brilliant scrape no-cache-check
```
You can use the arg "no-cache-check" to disable the saved cache checking. If you're using my contributions.cache I recommend using this arg.

You can use the arg "skip-downloaded-html-files" to skip re-checking of already downloaded html files. Useful if you stop-and-restart.

## To run web server (use cmd/terminal)
```bash
./scrape-brilliant web
```

The webserver is used for viewing what you scraped easily.


# Compiling

If you'd like to compile and run it instead of using the pre-made exe you can easily.

Download the repository by clicking the green button and clicking "Download Zip".

Installing Rust is super simple.
```
https://www.rust-lang.org/tools/install
```

After its installed. Run the command in the main directory (it contains src, Cargo.toml, etc..):
```bash
cargo build
```

After its built you can run it with these two commands.
```bash
cargo run -- web
```
OR
```bash
cargo run -- scrape
```