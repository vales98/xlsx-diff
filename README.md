# XLSX-DIFF

- `xlsx-diff` is a libray for comparing differences between two XLSX files using the Myers diff algorithm

## Base

- **Myers Diff Algorithm**: A data comparison tool that displays the smallest set of line-by-line deletions and insertions to transform one file into another.
- **calamine**: a fast library for parsing XLSX files.

## Usage
- **cargo.toml**
```
[dependencies]
xlsx-diff = "0.1.2"
```
- **example**
```rust
use std::path::PathBuf;
use std::error::Error;
use xlsx_diff::diff_xlsx;
fn main() -> Result<(), Box<dyn Error>> {
    let res = diff_xlsx(
        PathBuf::from("./old.xlsx"), // old file path
        PathBuf::from("./new.xlsx"), // new file path
        false); // output raw data
    match res {
        Ok(s) => println!("{}", serde_json::to_string_pretty(&s)?),
        Err(err) => println!("{:?}", err),
    }
    return Ok(());
}
```
- **output(serde to json)**
```json
{
  "added_sheets": [
    "Sheet1"
  ],
  "removed_sheets": [
    "100row"
  ],
  "modified_sheets": [
    {
      "sheet_name": "1000row",
      "diff": [
        {
          "op": "Delete",
          "old_index": 921,
          "new_index": null,
          "value": [
            921.0,
            "Belinda",
            "Partain",
            "Female",
            "United States",
            37.0,
            "15/10/2017",
            2564.0
          ]
        },
        {
          "op": "Insert",
          "old_index": null,
          "new_index": 921,
          "value": [
            921.0,
            "Belinda",
            "Karner",
            "Female",
            "United States",
            37.0,
            "15/10/2017",
            2564.0
          ]
        },
        {
          "op": "Insert",
          "old_index": null,
          "new_index": 933,
          "value": [
            924.0,
            "Libbie",
            "Dalby",
            "Female",
            "Great Britain",
            42.0,
            "21/05/2015",
            5489.0
          ]
        },
        {
          "op": "Delete",
          "old_index": 946,
          "new_index": null,
          "value": [
            946.0,
            "Roma",
            "Lafollette",
            "Female",
            "United States",
            34.0,
            "15/10/2017",
            2654.0
          ]
        }
      ]
    },
    {
      "sheet_name": "5000row",
      "diff": []
    }
  ],
  "data": null // raw data if you enable the raw_data option
}
```

