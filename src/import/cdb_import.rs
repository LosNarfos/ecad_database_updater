use serde::Deserialize;
use calamine::{open_workbook, Error, Xlsx, Reader, RangeDeserializerBuilder};


#[derive(Clone, Deserialize, Debug)]
pub struct PartFromCDB {
    #[serde(default, rename = "CDB No")]
    pub cdb_number: String,
    #[serde(default, rename = "SAP No")]
    pub sap_number: String,
    #[serde(default, rename = "Part Name")]
    pub part_name: String,
    #[serde(default, rename = "Value")]
    pub value: String,
    #[serde(default, rename = "Package")]
    pub package: String,
    #[serde(default, rename = "Temperature")]
    pub temperature: String,
    #[serde(default, rename = "Height")]
    pub height: String,
    #[serde(default, rename = "Library Ref")]
    pub library_ref: String,
    #[serde(default, rename = "Library Path")]
    pub library_path: String,
    #[serde(default, rename = "Footprint Ref")]
    pub footprint_ref1: String,
    #[serde(default, rename = "Footprint Path")]
    pub footprint_path1: String,
    #[serde(default, rename = "Footprint Ref 2")]
    pub footprint_ref2: String,
    #[serde(default, rename = "Footprint Path 2")]
    pub footprint_path2: String,
    #[serde(default, rename = "Altium state")]
    pub altium_state: String,
    #[serde(default, rename = "Description")]
    pub description: String,
    #[serde(default, rename = "Voltage")]
    pub voltage: String,
    #[serde(default, rename = "Tolerance")]
    pub tolerance: String,
    #[serde(default, rename = "Current")]
    pub current: String,
    #[serde(default, rename = "Power")]
    pub power: String,
    #[serde(default, rename = "Info1")]
    pub info1: String,
    #[serde(default, rename = "Info2")]
    pub info2: String,
    #[serde(default, rename = "Info3")]
    pub info3: String,
    #[serde(default, rename = "Info4")]
    pub info4: String,
    #[serde(default, rename = "Info5")]
    pub info5: String,
    #[serde(default, rename = "Info6")]
    pub info6: String,
    #[serde(default, rename = "Info7")]
    pub info7: String,
    #[serde(default, rename = "Info8")]
    pub info8: String,
    #[serde(default, rename = "Info9")]
    pub info9: String,
    #[serde(default, rename = "Info10")]
    pub info10: String,

    // not yet implemented on CDB side
    #[serde(default, rename = "CDB State")]
    pub cdb_state: String,
    #[serde(default, rename = "Manufacturer")]
    pub manufacturer: String,
    #[serde(default, rename = "Manufacturer_number")]
    pub manufacturer_number: String,
    #[serde(default, rename = "Category")]
    pub category: String,
    #[serde(default, rename = "Type")]
    pub part_type: String,
    #[serde(default, rename = "Help_URL")]
    pub help_url: String,
    #[serde(default, rename = "datasheet_URL")]
    pub datasheet_url: String,
    #[serde(default, rename = "Footprint Ref3")]
    pub footprint_ref3: String,
    #[serde(default, rename = "Footprint Ref4")]
    pub footprint_ref4: String,
    #[serde(default, rename = "Footprint Ref5")]
    pub footprint_ref5: String,
    #[serde(default, rename = "Footprint Path 3")]
    pub footprint_path3: String,
    #[serde(default, rename = "Footprint Path 4")]
    pub footprint_path4: String,
    #[serde(default, rename = "Footprint Path 5")]
    pub footprint_path5: String,
    #[serde(default, rename = "Model")]
    pub model: String,
    #[serde(default, rename = "Model Ref")]
    pub model_ref: String,
    #[serde(default, rename = "Model Path")]
    pub model_path: String,
    
}

pub fn read_cdb_export(filename: &str) -> Result<Vec<PartFromCDB>, Error> {
    let mut parts: Vec<PartFromCDB> = Vec::new();

    // open Excel file
    let path = format!("{}\\CDB_Export\\{}.xlsx", env!("CARGO_MANIFEST_DIR"), &filename);
    println!("  Reading in CDB-Excel File: {}", path);
    let mut workbook: Xlsx<_> = open_workbook(path)?;

    // open sheet in Excel file
    let range = workbook
        .worksheet_range("cdb_export")
        .ok_or(calamine::Error::Msg("Cannot find data worksheet"))??;

    let iter_result = RangeDeserializerBuilder::new().from_range(&range)?;
    
    // write to parts vector
    for (index, row) in iter_result.enumerate() {
        match row {
            Ok(row) => {
                parts.push(row);
            }
            Err(row) => println!("{}: {:?}", index + 2, row),
        }
    }
    Ok(parts)

}
 