use std::collections::{HashMap, HashSet};
use reqwest::Client; // make an HTTP connection to the host website
use scraper::{Html, Selector}; // parse the HTML content
use std::fs::{self, File};
use std::io::{self, Write, BufWriter, stdout, BufReader, Read, BufRead};

//noinspection RsMainFunctionNotFound
/// This is the main function of the program.
///
/// It is an asynchronous function that uses the Tokio runtime.
/// It makes an HTTP request to the website, parses the HTML content,
/// and writes the extracted data to a file.
///
/// # Arguments
///
/// * `None`
///
/// # Returns
///
/// * `None`
#[tokio::main] // This attribute sets up the Tokio runtime for your async main function
// async fn main() -> io::Result<()> {
async fn main() {


    let client = Client::new(); // create a new HTTP client

    // let user_website = get_input();

    let res = client.get("http://books.toscrape.com/") // make a GET request to the URL
        .send()
        .await // use .await here to wait for the Future to complete
        .unwrap(); // unwrap is like a try-catch

    let body = res.text()
        .await
        .unwrap(); // will return an HTML string

    // before we use the scraper library we have to convert this string into an scraper::Html *object*
    let document = Html::parse_document(&body);

    let wildcard_selector = Selector::parse("*").unwrap();
    let mut css_classes = HashSet::new();
    let mut css_ids = HashSet::new();
    let mut html_tags = HashSet::new();
    let mut tag_class_map = HashMap::new();

    for element in document.select(&scraper::Selector::parse("*").unwrap()) {
        process_elements(&element, &mut html_tags, &mut tag_class_map);
    }

    // let tag_class_map: HashMap<String, Vec<String>> = tag_class_map.into_iter()
    //     .map(|(tag, class_set)| (tag, class_set.into_iter().collect()))
    //     .collect();

    // All the CSS Class Selectors
    let mut class_list: Vec<_> = css_classes.into_iter().collect();
    let mut id_list: Vec<_> = css_ids.into_iter().collect();
    let mut tag_list: Vec<_> = html_tags.into_iter().collect();
    class_list.sort();
    id_list.sort();

    let user_selector_choice = id_or_class(class_list, id_list, tag_list, tag_class_map);
    println!("DEBUG: user_selector_choice == {}", user_selector_choice);

    let testing = Selector::parse(&user_selector_choice).unwrap();

    // println!("What part of the website do you want me to look at?");
    // let user_selector = get_input();
    //
    // println!("Which element of this page do you want to scrape?");
    // let user_element = get_input();
    //
    // println!("We have parsed the HTML. Now we are selecting the elements...");
    // let book_title_selector = Selector::parse(&user_selector).unwrap();
    // let book_price_selector = Selector::parse(&user_element).unwrap();
    //
    println!("What do you want to name the output file?");
    let user_file_name = get_input();
    // let mut file = create_file(&user_file_name);
    //
    // println!("We have selected the elements. Now we are writing the output to a file...");
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

    // Use a HashSet to prevent duplicate classes
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

    // Convert the HashSet to a Vec and insert it into tag_class_map
    let child_classes = class_set.into_iter().collect::<Vec<String>>();
    tag_class_map.insert(tag_name, child_classes);
}

fn id_or_class(class_list: Vec<String>, id_list: Vec<String>, tag_list: Vec<String>, tag_class_map: HashMap<String, Vec<String>>) -> String {

    let mut user_class_id_choice = String::new();

    // Ask the user if they want IDs or classes
    print!("Type 'classes' to view Classes and Type 'id' to view ID's and type 'tag' to view HTML Tag's (type 'quit' to exit): ");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut class_or_id = String::new();
    io::stdin()
        .read_line(&mut class_or_id)
        .expect("Failed to read line");
    class_or_id = class_or_id.trim().to_lowercase();

    // Process user input
    match class_or_id.as_str() {

        "classes" => {
            println!("\nCSS Classes found:\n");
            for (index, class) in class_list.iter().enumerate() {
                println!("{} - {}", index, class);
            }

            print!("Enter the class name or its index: ");
            io::stdout().flush().expect("Failed to flush stdout");

            let mut user_choice = String::new();
            io::stdin()
                .read_line(&mut user_choice)
                .expect("Failed to read line");
            let user_choice = user_choice.trim();

            // Check if user input is a digit and convert it to usize
            let class_selection = if user_choice.chars().all(char::is_numeric) {
                user_choice.parse::<usize>().ok()
                    .and_then(|index| class_list.get(index))
                    .map(|class| class.to_string())
            } else {
                // Check if it's a valid class name
                class_list.contains(&user_choice.to_string()).then(|| user_choice.to_string())
            };

            println!("DEBUG: class_selection == {:?}", class_selection);

            match class_selection {
                Some(class) => {
                    println!("You selected: {}", class);
                    class
                },
                None => {
                    println!("Invalid selection");
                    "Invalid".to_string()
                }
            }
        },

        "id" => {
            println!("\nID's found:\n");
            for (index, id) in id_list.iter().enumerate() {
                println!("{} - {}", index, id);
            }

            print!("Enter the ID name or its index: ");
            io::stdout().flush().expect("Failed to flush stdout");

            let mut user_choice = String::new();
            io::stdin()
                .read_line(&mut user_choice)
                .expect("Failed to read line");
            let user_choice = user_choice.trim();

            // Check if user input is a digit and convert it to usize
            let id_selection = if user_choice.chars().all(char::is_numeric) {
                user_choice.parse::<usize>().ok()
                    .and_then(|index| id_list.get(index))
                    .map(|id| id.to_string())
            } else {
                // Check if it's a valid ID name
                id_list.contains(&user_choice.to_string()).then(|| user_choice.to_string())
            };

            match id_selection {
                Some(id) => {
                    println!("You selected: {}", id);
                    id
                },
                None => {
                    println!("Invalid selection");
                    "Invalid".to_string()
                }
            }
        },

        "tag" => {
            // println!("\nHTML tag's found:\n");
            // for (index, tag) in tag_list.iter().enumerate() {
            //     println!("{} - {}", index, tag)
            // }

            println!("\nHTML tag's found:\n");
            for (tag, classes) in &tag_class_map {
                println!("- {}", tag);
                for class in classes {
                    println!("    - {:?}", class);
                }
            }

            print!("Enter the tag name or its index: ");
            io::stdout().flush().expect("Failed to flush stdout");

            let mut user_choice = String::new();
            io::stdin()
                .read_line(&mut user_choice)
                .expect("Failed to read line");
            let user_choice = user_choice.trim();

            // Check if user input is a digit and convert it to usize
            let tag_selection = if user_choice.chars().all(char::is_numeric) {
                user_choice.parse::<usize>().ok()
                    .and_then(|index| tag_list.get(index))
                    .map(|tag| tag.to_string())
            } else {
                // Check if it's a valid tag name
                tag_list.contains(&user_choice.to_string()).then(|| user_choice.to_string())
            };

            match tag_selection {
                Some(tag) => {
                    println!("You selected: {}", tag);
                    tag
                },
                None => {
                    println!("Invalid selection");
                    "Invalid".to_string()
                }
            }
        },

        "quit" => {
            println!("Exiting...");
            "quit".to_string()
        },

        _ => {
            "Invalid".to_string() // Return a string indicating invalid input
        },
    }

    // Ask which class or ID to grab
    // print!("Which one do you want to grab? ");
    // io::stdout().flush().expect("Failed to flush stdout");
    // io::stdin()
    //     .read_line(&mut user_class_id_choice)
    //     .expect("Failed to read line");
    // user_class_id_choice.trim().to_string()
}

/// Writes text data extracted from an HTML document to a file.
///
/// This function iterates over elements in the provided HTML document that match
/// the specified CSS selector. It then writes the text content of these elements
/// to the given file buffer.
///
/// # Arguments
///
/// * `document` - A reference to the `Html` document from which data is to be extracted.
///     This is the document that will be searched using the provided CSS selector.
///
/// * `selector` - A reference to a `Selector` that defines the CSS pattern used to
///     select elements from the `document`. The text content of these elements is what
///     will be written to the file.
///
/// * `file` - A mutable reference to a `BufWriter<File>` where the extracted text
///     will be written. Using a buffered writer improves efficiency when writing to
///     the file system.
///
/// # Returns
///
/// Returns an `io::Result<()>`. On successful execution, it returns `Ok(())`.
/// On failure, such as encountering an I/O error while writing to the file, it returns
/// an `io::Error`.
fn scrape_and_write_to_file(document: &Html, selector: &Selector, file: &mut BufWriter<File>) -> io::Result<()> {

    for element in document.select(selector) {
        let element_text = element.text().collect::<Vec<_>>().join(" ");
        println!("DEBUG: element_text == {:?}", element_text);

        // Clean the String
        let processed_text = clean_content(&element_text);

        writeln!(file, "{}", &processed_text)?;
    }
    Ok(())
}

/// This function will take user-input
/// It will then Trim any whitespaces
/// and return a String
///
/// # Arguments
///
/// * `None`
///
/// # Returns
///
/// * `String`
fn get_input() -> String {
    let mut input = String::new();

    println!("Give me some input");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input.trim().to_string()
}

/// This function will handle creating a file
///
/// # Arguments
///
/// * `file_name` - A reference to a `str` that defines the name of the file to be created
///
/// # Returns
///
/// Returns a `std::io::Result<BufWriter<File>>`. On successful execution, it returns `Ok(BufWriter<File>)`.
///
/// On failure, such as encountering an I/O error while writing to the file, it returns
/// an `io::Error`.
fn create_file(file_name: &str) -> std::io::Result<BufWriter<File>> {
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