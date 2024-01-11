use reqwest::Client; // make an HTTP connection to the host website
use scraper::{Html, Selector}; // parse the HTML content
use std::fs::File;
use std::io::{self, Write, BufWriter};

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
async fn main() {

    println!("We are SCRAPING!");

    let client = Client::new(); // create a new HTTP client

    println!("Which website would you like to scrape?");
    let user_website = get_input();


    println!("We are making a request to the website. Unwrapping the result...");
    let res = client.get("http://books.toscrape.com/") // make a GET request to the URL
        .send()
        .await // use .await here to wait for the Future to complete
        .unwrap(); // unwrap is like a try-catch

    println!("We have made a request to the website. Now we are reading the response...");
    let body = res.text()
        .await
        .unwrap(); // will return an HTML string

    // Printing out the HTML string
    println!("The HTML string is: {}", body);

    // before we use the scraper library we have to convert this string into an scraper::Html *object*
    println!("We have read the response. Now we are parsing the HTML...");
    let document = Html::parse_document(&body);

    println!("What part of the website do you want me to look at?");
    let user_selector = get_input();

    println!("Which element of this page do you want to scrape?");
    let user_element = get_input();

    println!("We have parsed the HTML. Now we are selecting the elements...");
    let book_title_selector = Selector::parse(&user_selector).unwrap();
    let book_price_selector = Selector::parse(&user_element).unwrap();

    println!("What do you want to name the output file?");
    let user_file_name = get_input();
    let mut file = create_file(&user_file_name);

    println!("We have selected the elements. Now we are writing the output to a file...");
    let mut file = create_file(&user_file_name).expect("Failed to create file");
    scrape_and_write_to_file(&document, &book_title_selector, &mut file)
        .expect("Unable to write book titles");
    scrape_and_write_to_file(&document, &book_price_selector, &mut file)
        .expect("Unable to write book prices");

    println!("We have written the output to a file. We are done scraping!");
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
        writeln!(file, "{}", element_text)?;
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