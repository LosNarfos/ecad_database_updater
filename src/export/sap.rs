use csv::{QuoteStyle, WriterBuilder};
use serde::Serialize;


use crate::Parts;

#[derive(Debug, Serialize, Default, Clone)]
pub struct Record {
    #[serde(default, rename = "Default Stock Code")]
    pub default_stock_id: String,
    #[serde(default, rename = "SAP_MatNo_HTN")]
    pub sap_mat_no_htn: String,
    #[serde(default, rename = "SAP_PartNoManufacterer")]
    pub part_no_manufacturer: String,
    #[serde(default, rename = "SAP_Manufacterer")]
    pub manufacturer: String,
    #[serde(default, rename = "SAP_Plant")]
    pub plant: String,
    #[serde(default, rename = "SAP_Mat_status")]
    pub mat_status: String,
    #[serde(default, rename = "SAP_StandardMat")]
    pub standard_mat: String,
    #[serde(default, rename = "SAP_Labor")]
    pub labor: String,
    #[serde(default, rename = "SAP_Washable")]
    pub washable: String,
    #[serde(default, rename = "SAP_MatGroup")]
    pub mat_group: String,
    #[serde(default, rename = "SAP_MatShortText")]
    pub mat_short_text: String,
    #[serde(default, rename = "SAP_SELCD")]
    pub selection_code: String,
    #[serde(default, rename = "SAP_MTTF_ISO")]
    pub mttf: String,
    #[serde(default, rename = "SAP_Device_Marking")]
    pub device_marking: String,
    #[serde(default, rename = "SAP_Price_Info")]
    pub price_info: String,
}

pub fn export(parts: &Parts) {
        
    let mut wtr = WriterBuilder::new()
        .quote_style(QuoteStyle::Always)
        .has_headers(false)
        .from_path("SAP_Export\\Extract_Zuken4SAP.csv").unwrap();

    // header must be written manually
    let header = [
        "Default Stock Code",
        "SAP_MatNo_HTN",
        "SAP_PartNoManufacterer",
        "SAP_Manufacterer",
        "SAP_Plant",
        "SAP_Mat_status",
        "SAP_StandardMat",
        "SAP_Labor",
        "SAP_Washable",
        "SAP_MatGroup",
        "SAP_MatShortText",
        "SAP_SELCD",
        "SAP_MTTF_ISO",
        "SAP_Device_Marking",
        "SAP_Price_Info"
    ];
    wtr.write_record(&header).unwrap();

    // The first 2 entries are ignored by the SAP importer (csv-file from Zuken has additional column infos here)
    wtr.write_record(&["", "", "", "", "", "", "", "", "", "", "", "", "", "", ""]).unwrap();
    wtr.write_record(&["", "", "", "", "", "", "", "", "", "", "", "", "", "", ""]).unwrap();

    // now for the real data
    for part in parts.capacitor.iter() { wtr.serialize( Record { default_stock_id: part.sap_number.clone(), ..Default::default() }).unwrap(); }
    for part in parts.connector.iter() { wtr.serialize( Record { default_stock_id: part.sap_number.clone(), ..Default::default() }).unwrap(); }
    for part in parts.diode.iter() { wtr.serialize( Record { default_stock_id: part.sap_number.clone(), ..Default::default() }).unwrap(); }
    for part in parts.ic.iter() { wtr.serialize( Record { default_stock_id: part.sap_number.clone(), ..Default::default() }).unwrap(); }
    for part in parts.inductor.iter() { wtr.serialize( Record { default_stock_id: part.sap_number.clone(), ..Default::default() }).unwrap(); }
    for part in parts.mechanic.iter() { wtr.serialize( Record { default_stock_id: part.sap_number.clone(), ..Default::default() }).unwrap(); }
    for part in parts.opto.iter() { wtr.serialize( Record { default_stock_id: part.sap_number.clone(), ..Default::default() }).unwrap(); }
    for part in parts.other.iter() { wtr.serialize( Record { default_stock_id: part.sap_number.clone(), ..Default::default() }).unwrap(); }
    for part in parts.resistor.iter() { wtr.serialize( Record { default_stock_id: part.sap_number.clone(), ..Default::default() }).unwrap(); }
    for part in parts.transistor.iter() { wtr.serialize( Record { default_stock_id: part.sap_number.clone(), ..Default::default() }).unwrap(); }

    // send everthing to file
    wtr.flush().unwrap();




}
