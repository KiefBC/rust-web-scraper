// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::collections::{HashMap, HashSet};
use reqwest::Client; // make an HTTP connection to the host website
use scraper::{Html, Selector}; // parse the HTML content
use std::fs::{File};
use std::io::{self, Write, BufWriter, BufRead};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tokio::main] // This attribute sets up the Tokio runtime for your async main function
async fn main() {

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    let client = Client::new(); // create a new HTTP client

    let res = client.get("https://books.toscrape.com/") // make a GET request to the URL
        .send()
        .await // use .await here to wait for the Future to complete
        .unwrap(); // unwrap is like a try-catch

    let body = res.text()
        .await
        .unwrap(); // will return an HTML string

    // before we use the scraper library we have to convert this string into an scraper::Html *object*
    let document = Html::parse_document(&body);
    let mut html_tags = HashSet::new();
    let mut tag_class_map = HashMap::new();

    for element in document.select(&Selector::parse("*").unwrap()) {
        process_elements(&element, &mut html_tags, &mut tag_class_map);
    }

    let user_selector_choice = id_or_class(tag_class_map);

    let testing = Selector::parse(&user_selector_choice).unwrap();

    // println!("We have parsed the HTML. Now we are selecting the elements...");
    // let book_title_selector = Selector::parse(&user_selector).unwrap();
    // let book_price_selector = Selector::parse(&user_element).unwrap();

    println!("What do you want to name the output file?");
    let user_file_name = format!("{}.txt", "output");
    let mut file = create_file(&user_file_name).expect("Failed to create file");

    // scrape_and_write_to_file(&document, &book_title_selector, &mut file)
    //     .expect("Unable to write book titles");

    scrape_and_write_to_file(&document, &testing, &mut file)
        .expect("Unable to write book titles");
    // scrape_and_write_to_file(&document, &book_price_selector, &mut file)
    //     .expect("Unable to write book prices");
    //
    println!("We have written the output to a file. We are done scraping!");
}
fn process_elements(element: &scraper::element_ref::ElementRef, html_tags: &mut HashSet<String>, tag_class_map: &mut HashMap<String, Vec<String>>) {
    // Extracting HTML tag names
    let tag_name = element.value().name().to_string();
    html_tags.insert(tag_name.clone());

    // Use a HashSet to prevent duplicate classes for this element
    let mut class_set = HashSet::new();

    // Include classes of the element itself
    if let Some(class_list) = element.value().attr("class") {
        for class in class_list.split_whitespace() {
            class_set.insert(class.to_string());
        }
    }

    // Include classes of the element's children
    for node in element.children() {
        if let Some(child) = node.value().as_element() {
            if let Some(class_list) = child.attr("class") {
                for class in class_list.split_whitespace() {
                    class_set.insert(class.to_string());
                }
            }
        }
    }

    // Append new classes to the existing vector in the map for this tag
    let entry = tag_class_map.entry(tag_name).or_insert_with(Vec::new);
    for class in class_set {
        if !entry.contains(&class) {
            entry.push(class);
        }
    }
}
fn id_or_class(tag_class_map: HashMap<String, Vec<String>>) -> String {
    let mut user_selector_choice = String::new();

    let explain_format = r#"
The following is a list of all the HTML tags and CSS classes that we found on the website.

You can use this information to help you decide what you want to scrape.

"- <tag_name>"
    * <class_name>
    * <class_name>
    * <class_name>"

====================================
====================================
    "#;

    println!("{}", explain_format);

    for (tag, classes) in &tag_class_map {
        println!("- {}", tag);
        for class in classes {
            println!("    * {:?}", class);
        }
    }

    println!("\nWhich Tag or Class?");
    io::stdin()
        .read_line(&mut user_selector_choice)
        .expect("Failed to read input");

    user_selector_choice = user_selector_choice.trim().to_string();

    // Check if the user input is a tag or a class
    if tag_class_map.contains_key(&user_selector_choice) {
        user_selector_choice = format!("{}[class]", user_selector_choice);
    } else {
        user_selector_choice = format!(".{}", user_selector_choice);
    }

    user_selector_choice
}
fn scrape_and_write_to_file(document: &Html, selector: &Selector, file: &mut BufWriter<File>) -> io::Result<()> {

    for element in document.select(selector) {
        let element_text = element.text().collect::<Vec<_>>().join(" ");

        // Clean the String
        let processed_text = clean_content(&element_text);

        writeln!(file, "{}", &processed_text)?;
    }
    Ok(())
}
fn create_file(file_name: &str) -> io::Result<BufWriter<File>> {
    let file = File::create(file_name)?;
    let file = BufWriter::new(file);

    println!("File created");

    Ok(file)
}
fn clean_content(content: &str) -> String {

    // Process the element_text to remove empty lines
    let processed_content = content
        .lines()
        .filter(|line| !line.trim().is_empty()) // Filter out empty or whitespace-only lines
        .map(|line| line.trim()) // Trim leading and trailing whitespace
        .collect::<Vec<&str>>()
        .join("\n");

    processed_content.to_string()
}
