extern crate calamine;
extern crate serde;
extern crate serde_json;
use calamine::{
    open_workbook, open_workbook_from_rs, Data, Range, RangeDeserializerBuilder, Reader, Xlsx,
};
use std::io::Cursor;
// use serde::Serialize;
use serde::{Serialize, Serializer};
use std::path::PathBuf;
use std::{collections::HashMap, error::Error};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct SerializableData(Data);
impl Eq for SerializableData {}
impl Serialize for SerializableData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match &self.0 {
            Data::Empty => serializer.serialize_none(),
            Data::String(s) => serializer.serialize_str(s),
            Data::Float(f) => serializer.serialize_f64(*f),
            Data::Int(i) => serializer.serialize_i64(*i),
            Data::Bool(b) => serializer.serialize_bool(*b),
            Data::DateTime(dt) => serializer.serialize_str(&dt.to_string()),
            Data::Error(e) => serializer.serialize_str(&format!("{:?}", e)),
            Data::DateTimeIso(dt) => serializer.serialize_str(&dt.to_string()),
            Data::DurationIso(d) => serializer.serialize_str(&d.to_string()),
        }
    }
}

type RowData = Vec<SerializableData>;
pub type SheetData = Vec<RowData>;
pub type WorkbookData = HashMap<String, SheetData>;
#[derive(serde::Serialize)]
pub struct CalamineWorkbook {
    pub sheet_names: Vec<String>,
    pub data: WorkbookData,
}

pub enum FileLike {
    Path(PathBuf),
    Bytes(Vec<u8>),
}

pub fn load_workbook(file: &FileLike) -> Result<CalamineWorkbook, Box<dyn Error>> {
    match file {
        FileLike::Path(path) => load_by_path(path),
        FileLike::Bytes(bytes) => load_by_rs(bytes),
    }
}

fn load_by_path(file: &PathBuf) -> Result<CalamineWorkbook, Box<dyn Error>> {
    let mut workbook: Xlsx<_> = open_workbook(file)?;
    Ok(build_workbook(&mut workbook)?)
}

pub fn load_by_rs(file: &[u8]) -> Result<CalamineWorkbook, Box<dyn Error>> {
    let mut workbook: Xlsx<_> = open_workbook_from_rs(Cursor::new(file))?;
    Ok(build_workbook(&mut workbook)?)
}

fn build_workbook<T: std::io::Read + std::io::Seek>(workbook: &mut Xlsx<T>) -> Result<CalamineWorkbook, Box<dyn Error>> {
    let mut calamine_workbook = CalamineWorkbook {
        sheet_names: workbook.sheet_names().to_vec(),
        data: HashMap::new(),
    };
    for sheet_name in workbook.sheet_names() {
        if let Ok(sheet) = workbook.worksheet_range(&sheet_name) {
            let sheet_data = deserialize_sheet(&sheet)?;
            calamine_workbook.data.insert(sheet_name, sheet_data);
        }
    }
    Ok(calamine_workbook)
}

fn deserialize_sheet(sheet: &Range<Data>) -> Result<SheetData, Box<dyn Error>> {
    let mut iter = RangeDeserializerBuilder::new()
        .has_headers(false)
        .from_range(&sheet)?;
    let mut sheet_data: SheetData = Vec::new();
    while let Some(result) = iter.next() {
        let row: Vec<Data> = result?;
        let mut row_data: RowData = Vec::new();
        for (index, cell) in row.into_iter().enumerate() {
            row_data.push(SerializableData(cell));
        }
        sheet_data.push(row_data);
    }
    Ok(sheet_data)
}
