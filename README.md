# replace-from-sheets
A simple Rust tool to replace words in a text file from any Google Spreadsheet.
It takes two columns in a Google Spreadsheet and uses them to replace words in any text file.

First column: check which word should be replaced.

Second column: replacing word

| Column A  | Column B |
| ------------- | ------------- |
| Aple  | Apple  |
| Starwbery  | Strawberry  |

Words on the same line will get replaced.

**Having 2 or more words in a cell works too, but both cells need to have the same amount of words**

## Getting Started

Inside the src folder there is a init.json file which needs to be edited to your own spreadsheet ID, your desired columns and your own API.

Then navigate to the folder path in your terminal and type ```replace-from-sheets FILENAME.txt```
The text file you want to edit needs to be in the same folder.

### Prerequisites

To build yourself you only need Rust. All Cargo dependencies will be downloaded on building.

Get your own Google Spreadsheet API or any other Google API from any account. It doesn't matter. The API is only used to read the columns.

**Example init.json**
```
{
    "spreadsheet_id": "1234567",
    "column1": "Sheet1!A1:A",
    "column2": "Sheet1!B1:B",
    "api_key": "123abc123abc"
}
```

### Installing

TBA
