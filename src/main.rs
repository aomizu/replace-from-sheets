use serde_json::Value;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // get arguments from command line

    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        panic!("Too many arguments! Example: replace-from-sheets FILENAME.txt");
    } else if args.len() == 1 {
        panic!("Missing argument! Example: replace-from-sheets FILENAME.txt");
    }
    let filename = &args[1];

    // struct Init for the values of init.json

    #[derive(Debug, serde::Deserialize)]
    struct Init {
        spreadsheet_id: String,
        column1: String,
        column2: String,
        api_key: String,
    }

    let file_json = File::open("init.json").expect("Couldn't open init.json file!");
    let init: Init = serde_json::from_reader(file_json).unwrap();

    let spreadsheet_id = init.spreadsheet_id;
    let column1 = init.column1;
    let column2 = init.column2;
    let api_key = init.api_key;

    // Build the URL to call the Sheets API to retrieve the column data
    let url1 = format!(
        "https://sheets.googleapis.com/v4/spreadsheets/{}/values/{}?key={}",
        spreadsheet_id, column1, api_key
    );

    let url2 = format!(
        "https://sheets.googleapis.com/v4/spreadsheets/{}/values/{}?key={}",
        spreadsheet_id, column2, api_key
    );

    // Call the Sheets API to retrieve the column data
    let response1 = reqwest::Client::new()
        .get(&url1)
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    let response2 = reqwest::Client::new()
        .get(&url2)
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    let first_column = get_column_values(response1);
    let second_column = get_column_values(response2);

    let new_contents = edit_file(first_column, second_column, &filename);
    let mut file = File::create(&filename)?;
    file.write_all(new_contents.as_bytes())
        .expect("Can't write to new file.");

    Ok(())
}

fn get_column_values(response: Value) -> Vec<String> {
    let mut v: Vec<String> = Vec::new();
    if let Some(values) = response.get("values").and_then(|v| v.as_array()) {
        for row in values {
            for value in row.as_array().unwrap() {
                // push value into vector
                let value = value.to_string().replace("\"", "");
                if value.to_string().split_whitespace().count() > 1 {
                    for word in value.to_string().split_whitespace() {
                        v.push(String::from(word));
                    }
                } else {
                    v.push(value.to_string());
                }
            }
        }
    }
    v
}

fn edit_file(vector1: Vec<String>, vector2: Vec<String>, filename: &String) -> String {
    let mut file = File::open(filename).expect("Unable to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read file");
    for (word1, word2) in vector1.iter().zip(vector2.iter()) {
        contents = contents.replace(word1, word2);
    }
    contents
}
