use std::error::Error;
use std::path::PathBuf;
extern crate serde_json;
pub mod diff;
pub mod parse;
use diff::DiffItem;
use parse::{CalamineWorkbook, FileLike, SerializableData,WorkbookData};
use serde::Serialize;

#[derive(Serialize)]
struct ModifiedSheet {
    sheet_name: String,
    diff: Vec<DiffItem<Vec<SerializableData>>>,
}
#[derive(Serialize)]
struct OriginWorkbookData {
    sheet_names: Vec<String>,
    data: Option<WorkbookData>
}

#[derive(Serialize)]
struct OriginData {
    old: OriginWorkbookData,
    new: OriginWorkbookData,
}
#[derive(Serialize)]
pub struct DiffResult {
    added_sheets: Vec<String>,
    removed_sheets: Vec<String>,
    no_change_sheets: Vec<String>,
    modified_sheets: Vec<ModifiedSheet>,
    data: OriginData
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
    let wb_old: parse::CalamineWorkbook = parse::load_workbook(&FileLike::Path(old_file_path))?;
    let wb_new = parse::load_workbook(&FileLike::Path(new_file_path))?;
    return build_diff(wb_old, wb_new, raw_data);
}
pub fn diff_xlsx_from_u8(
    old_file: &[u8],
    new_file: &[u8],
    raw_data: bool,
) -> Result<DiffResult, Box<dyn Error>> {
    let wb_old: CalamineWorkbook = parse::load_workbook(&FileLike::Bytes(old_file.to_vec()))?;
    let wb_new = parse::load_workbook(&FileLike::Bytes(new_file.to_vec()))?;
    return build_diff(wb_old, wb_new, raw_data);
}

pub fn build_diff(
    wb_old: CalamineWorkbook,
    wb_new: CalamineWorkbook,
    raw_data: bool,
) -> Result<DiffResult, Box<dyn Error>> {
    let mut modified_sheets: Vec<ModifiedSheet> = Vec::new();
    let mut no_change_sheets: Vec<String> = Vec::new();
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
    for sheet_name in wb_old.sheet_names.iter() {
        if wb_new.sheet_names.contains(sheet_name) {
            let wb_old_first_sheet_data: &Vec<Vec<SerializableData>> =
                wb_old.data.get(&sheet_name.to_string()).unwrap();
            let wb_new_first_sheet_data = wb_new.data.get(&sheet_name.to_string()).unwrap();
            let res = diff::myers_diff(&wb_old_first_sheet_data, &wb_new_first_sheet_data);
            if res.len() == 0 {
                no_change_sheets.push(sheet_name.to_string());
            } else {
                modified_sheets.push(ModifiedSheet {
                    sheet_name: sheet_name.to_string(),
                    diff: res,
                });
            }
        }
    }
    return Ok(DiffResult {
        added_sheets,
        removed_sheets,
        modified_sheets,
        no_change_sheets,
        data: OriginData{
            old: OriginWorkbookData{
                sheet_names: wb_old.sheet_names,
                data: if raw_data {Some(wb_old.data)} else {None}
            },
            new: OriginWorkbookData{
                sheet_names: wb_new.sheet_names,
                data: if raw_data {Some(wb_new.data)} else {None}
            }
        }
    });
}
