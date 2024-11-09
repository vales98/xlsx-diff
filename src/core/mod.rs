use std::error::Error;
use std::path::PathBuf;
extern crate serde_json;
mod diff;
mod parse;
use diff::DiffItem;
use parse::CalamineWorkbook;
use parse::{FileLike, SerializableData};
use serde::Serialize;
#[derive(Serialize)]
struct ModifiedSheet {
    sheet_name: String,
    diff: Vec<DiffItem<Vec<SerializableData>>>,
}
#[derive(Serialize)]
struct OriginData {
    old: CalamineWorkbook,
    new: CalamineWorkbook,
}
#[derive(Serialize)]
pub struct DiffResult {
    added_sheets: Vec<String>,
    removed_sheets: Vec<String>,
    modified_sheets: Vec<ModifiedSheet>,
    data: Option<OriginData>,
}
/**
 * Compare two xlsx files
 * @param old_file_path old file path
 * @param new_file_path new file path
 * @param raw_data output raw data
 * @param header_row todo
 * @returns diff result
 */
pub fn diff_xlsx(
    old_file_path: PathBuf,
    new_file_path: PathBuf,
    raw_data: bool,
    // header_row: usize,
) -> Result<DiffResult, Box<dyn Error>> {
    let mut modified_sheets: Vec<ModifiedSheet> = vec![];
    // file path
    let wb_old: parse::CalamineWorkbook = parse::load_workbook(&FileLike::Path(old_file_path))?;
    let wb_new = parse::load_workbook(&FileLike::Path(new_file_path))?;
    // find added and removed sheets
    let added_sheets: Vec<String> = wb_new
        .sheet_names
        .iter()
        .filter(|sheet_name| !wb_old.sheet_names.contains(*sheet_name))
        .map(|sheet_name| sheet_name.to_string())
        .collect();
    let removed_sheets: Vec<String> = wb_old
        .sheet_names
        .iter()
        .filter(|sheet_name| !wb_new.sheet_names.contains(*sheet_name))
        .map(|sheet_name| sheet_name.to_string())
        .collect();
    // find modified sheets
    for sheet_name in wb_old.sheet_names.iter() {
        if wb_new.sheet_names.contains(sheet_name) {
            let wb_old_first_sheet_data: &Vec<Vec<SerializableData>> =
                wb_old.data.get(&sheet_name.to_string()).unwrap();
            let wb_new_first_sheet_data = wb_new.data.get(&sheet_name.to_string()).unwrap();
            let res = diff::myers_diff(&wb_old_first_sheet_data, &wb_new_first_sheet_data);
            modified_sheets.push(ModifiedSheet {
                sheet_name: sheet_name.to_string(),
                diff: res,
            });
        }
    }
    return Ok(DiffResult {
        added_sheets,
        removed_sheets,
        modified_sheets,
        data: if raw_data {
            Some(OriginData {
                old: wb_old,
                new: wb_new,
            })
        } else {
            None
        },
    });
}
