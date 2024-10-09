use clap::Parser;
use deunicode::deunicode;
use regex::Regex;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self};

#[derive(Parser)]
#[command(
    name = "mop",
    version,
    author,
    about = "Clean CSV column names",
    long_about = r#"✨🧹✨ mop is a command-line tool that reads a CSV file, cleans and standardizes the column names, and outputs the modified CSV to stdout.

It removes special characters, transliterates Unicode characters to ASCII, replaces spaces with underscores, and ensures that all column names are unique."#,
    after_help = r#"EXAMPLES:

  mop data.csv > cleaned_data.csv

  mop data.csv | some_other_command

  cat data.csv | mop - > cleaned_data.csv"#,
)]
struct Cli {
    /// CSV file to process (reads from stdin if not provided)
    file: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    // Determine the input source
    let reader: Box<dyn std::io::Read> = match cli.file.as_deref() {
        Some("-") => Box::new(std::io::stdin()),
        Some(file_path) => {
            let file = File::open(file_path).unwrap_or_else(|e| {
                eprintln!("Error opening file {}: {}", file_path, e);
                std::process::exit(1);
            });
            Box::new(file)
        }
        None => {
            // Read from stdin if no file is provided
            Box::new(std::io::stdin())
        }
    };

    // Create a flexible CSV reader
    let mut reader = csv::ReaderBuilder::new()
        .flexible(true)
        .has_headers(true)
        .from_reader(reader);

    // Get the headers
    let headers = reader.headers().expect("Could not read headers").clone();

    // Clean the headers
    let mut cleaned_headers = clean_headers(headers.iter());

    // Initialize max_fields with the number of headers
    let mut max_fields = cleaned_headers.len();

    // Read all records into a vector, tracking max_fields
    let mut records = Vec::new();

    for result in reader.records() {
        let record = result.expect("Could not read record");
        if record.len() > max_fields {
            max_fields = record.len();
        }
        records.push(record);
    }

    // If there are more fields than headers, add extra header names
    if max_fields > cleaned_headers.len() {
        let mut extra_header_counter = 1;
        while cleaned_headers.len() < max_fields {
            let mut extra_header = format!("x_{}", extra_header_counter);
            // Ensure the extra header name is unique
            while cleaned_headers.contains(&extra_header) {
                extra_header_counter += 1;
                extra_header = format!("x_{}", extra_header_counter);
            }
            cleaned_headers.push(extra_header);
            extra_header_counter += 1;
        }
    }

    // Create a CSV writer to stdout
    let stdout = io::stdout();
    let handle = stdout.lock();
    let mut writer = csv::WriterBuilder::new()
        .has_headers(true)
        .from_writer(handle);

    // Write the cleaned headers
    writer
        .write_record(&cleaned_headers)
        .expect("Could not write headers");

    // Write the records, adjusting fields to match max_fields
    for record in records {
        let mut record_fields: Vec<&str> = record.iter().collect();

        if record_fields.len() < max_fields {
            // Pad with empty strings
            record_fields.resize(max_fields, "");
        }

        writer
            .write_record(&record_fields)
            .expect("Could not write record");
    }

    writer.flush().expect("Could not flush writer");
}


fn clean_headers<'a, I>(headers: I) -> Vec<String>
where
    I: Iterator<Item = &'a str>,
{
    let mut seen = HashSet::new();
    let mut result = Vec::new();
    let mut empty_counter = 1;

    for header in headers {
        let mut name = header.trim().to_string(); // Convert to owned String
        if name.is_empty() {
            name = format!("x_{}", empty_counter); // Own the String
            empty_counter += 1;
        }
        let mut cleaned = clean_name(&name);
        let original_cleaned = cleaned.clone();
        let mut counter = 1;

        // Ensure uniqueness
        while seen.contains(&cleaned) {
            counter += 1;
            cleaned = format!("{}_{}", original_cleaned, counter);
        }
        seen.insert(cleaned.clone());
        result.push(cleaned);
    }

    result
}

fn clean_name(name: &str) -> String {
    // Transliterate to ASCII
    let mut cleaned = deunicode(name).to_lowercase();

    // Replace non-alphanumeric characters with underscores
    let re = Regex::new(r"[^a-z0-9]+").unwrap();
    cleaned = re.replace_all(&cleaned, "_").to_string();

    // Remove leading and trailing underscores
    cleaned = cleaned.trim_matches('_').to_string();

    // Ensure the name is not empty
    if cleaned.is_empty() {
        cleaned = "x".to_string();
    }

    cleaned
}

#[test]
fn test_headers_with_whitespace() {
    let headers = vec!["  Name  ", "  Age", "Location  ", "  "];
    let cleaned = clean_headers(headers.iter().map(|s| *s));
    assert_eq!(cleaned, vec!["name", "age", "location", "x_1"]);
}

#[test]
fn test_headers_clean_to_same_name() {
    let headers = vec!["Name", "name", "NAME"];
    let cleaned = clean_headers(headers.iter().map(|s| *s));
    assert_eq!(cleaned, vec!["name", "name_2", "name_3"]);
}
#[test]
fn test_headers_with_numbers() {
    let headers = vec!["123", "456abc", "abc123", "123abc456"];
    let cleaned = clean_headers(headers.iter().map(|s| *s));
    assert_eq!(cleaned, vec!["123", "456abc", "abc123", "123abc456"]);
}
#[test]
fn test_data_rows_with_extra_fields() {
    let data = "name,age\nAlice,30,Engineer\nBob,25,Designer,New York";
    let mut reader = csv::ReaderBuilder::new()
        .flexible(true)
        .has_headers(true)
        .from_reader(data.as_bytes());

    let headers = reader.headers().unwrap().clone();
    let mut cleaned_headers = clean_headers(headers.iter());

    let mut max_fields = cleaned_headers.len();
    let mut records = Vec::new();

    for result in reader.records() {
        let record = result.unwrap();
        if record.len() > max_fields {
            max_fields = record.len();
        }
        records.push(record);
    }

    if max_fields > cleaned_headers.len() {
        let num_extra = max_fields - cleaned_headers.len();
        for i in 1..=num_extra {
            cleaned_headers.push(format!("x_{}", i));
        }
    }

    assert_eq!(cleaned_headers, vec!["name", "age", "x_1", "x_2"]);
    assert_eq!(records.len(), 2);
}
#[test]
fn test_data_rows_with_fewer_fields() {
    let data = "name,age,location\nAlice\nBob,25";
    let mut reader = csv::ReaderBuilder::new()
        .flexible(true)
        .has_headers(true)
        .from_reader(data.as_bytes());

    let headers = reader.headers().unwrap().clone();
    let cleaned_headers = clean_headers(headers.iter());

    let mut records = Vec::new();
    for result in reader.records() {
        let record = result.unwrap();
        records.push(record);
    }

    assert_eq!(cleaned_headers, vec!["name", "age", "location"]);
    assert_eq!(records.len(), 2);
}
#[test]
fn test_headers_unicode_normalization() {
    let headers = vec!["Café", "Café", "München", "Munchen"]; // The second one uses combining accent
    let cleaned = clean_headers(headers.iter().map(|s| *s));
    assert_eq!(cleaned, vec!["cafe", "cafe_2", "munchen", "munchen_2"]);
}
#[test]
fn test_headers_with_leading_digits() {
    let headers = vec!["123abc", "456def"];
    let cleaned = clean_headers(headers.iter().map(|s| *s));
    assert_eq!(cleaned, vec!["123abc", "456def"]);
}
#[test]
fn test_headers_clean_to_empty() {
    let headers = vec!["!!!", "###", "$$$"];
    let cleaned = clean_headers(headers.iter().map(|s| *s));
    assert_eq!(cleaned, vec!["x", "x_2", "x_3"]);
}
#[test]
fn test_headers_duplicates_after_cleaning() {
    let headers = vec!["First Name", "First-Name", "First@Name"];
    let cleaned = clean_headers(headers.iter().map(|s| *s));
    assert_eq!(cleaned, vec!["first_name", "first_name_2", "first_name_3"]);
}
#[test]
fn test_headers_mixed_case_numbers() {
    let headers = vec!["ID", "userID", "User_id", "userID1"];
    let cleaned = clean_headers(headers.iter().map(|s| *s));
    assert_eq!(cleaned, vec!["id", "userid", "user_id", "userid1"]);
}
#[test]
fn test_headers_accented_characters() {
    let headers = vec!["à", "á", "â", "ä", "ã"];
    let cleaned = clean_headers(headers.iter().map(|s| *s));
    assert_eq!(cleaned, vec!["a", "a_2", "a_3", "a_4", "a_5"]);
}
#[test]
fn test_empty_csv_file() {
    let data = "";
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(data.as_bytes());

    let headers_result = reader.headers();

    // Check that headers_result is Ok
    assert!(headers_result.is_ok());

    // Check that headers are empty
    let headers = headers_result.unwrap();
    assert!(headers.is_empty());

    // The program should handle this case without crashing
}

// Note: it is hard to know if a csv has no headers because this package does allow for csv headers that are numbers
// It is assumed that the csv will always have headers.