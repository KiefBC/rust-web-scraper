# Rust Web Scraper

## Overview

This Rust project is a web scraping tool designed to extract data from websites. It utilizes asynchronous programming with the Tokio runtime, making it efficient for network operations. The application uses `reqwest` for making HTTP requests and `scraper` for parsing the HTML content.

## Features

- Asynchronous web scraping.
- User input for target website.
- HTML parsing and data extraction.
- Writing extracted data to a file.

## Prerequisites

- [Rust](https://www.rust-lang.org/) programming environment.
- `reqwest` and `scraper` crates.

## Usage

To use this web scraper:

1. Compile and run the Rust program.
2. Enter the URL of the website you want to scrape when prompted.
3. The program will fetch and parse the website's HTML content.
4. Extracted data will be written to a specified output file.

## Example

```shell
$ cargo run
We are SCRAPING!
Which website would you like to scrape?
[Input the website URL]
[Input the HTML parent element you want to scrape]
[Input the HTML element attribute you want to scrape]
[Input the output file name]
"Output file created"
``` 

