#![allow(dead_code)]
#[allow(unused_variables)]

use serde_derive::Deserialize;
use std::fs;
use std::process::exit;
use toml;
use calamine::{open_workbook, Error, Xlsx, Reader, RangeDeserializerBuilder};

const COLUMNS: [&str; 2] = ["label", "value"];


#[warn(unused_variables)]

#[derive(Deserialize)]
struct DbConfig {
    database: Database,
    logging: Logging,
}

#[derive(Deserialize)]
struct Database {
    connection_driver: String,
    server_name: String,
    name: String,
    uid: String,
    password: String
}

#[derive(Debug)]
#[derive(Deserialize)]
struct Logging {
    logfile_name: String,
}

struct PartByCDB {
    cdb_no: String,
    sap_no: String,
    part_name: String,
    value: String,
    description: String,
    package: String,
    temperature: String,
    height: String,
    part_type: String,
    voltage: String,
    current: String,
    power: String,
    tolerance: String,
    info1: String,
    info2: String,
    info3: String,
    info4: String,
    info5: String,
    library_ref: String,
    library_path: String,
    footprint_ref_1: String,
    footprint_ref_2: String,
    footprint_ref_3: String,
    footprint_ref_4: String,
    footprint_ref_5: String,
    footprint_path_2: String,
    footprint_path_1: String,
    footprint_path_3: String,
    footprint_path_4: String,
    footprint_path_5: String
}

struct PartBySAP {
    default_stock_code: String,
    mat_no_htn: String,
    part_no_manufacturer: String,
    manufacturer: String,
    plant: String,
    mat_status: String,
    standard_material: String,
    labor: String,
    washable: String,
    mat_group: String,
    mat_shorttext: String,
    selection_code: String,
    mttf: String,
    marking: String,
    price_info: String,
}

#[derive(Debug, Deserialize)]
struct DataType {
    label: String,
    value: f64,
}

fn example() -> Result<(), Error> {
    let path = format!("{}/tests/temperature.xlsx", env!("CARGO_MANIFEST_DIR"));
    let mut workbook: Xlsx<_> = open_workbook(path)?;
    let range = workbook.worksheet_range("Sheet1")
        .ok_or(Error::Msg("Cannot find 'Sheet1'"))??;

    let mut iter = RangeDeserializerBuilder::new()
        .has_headers(false)
        .from_range(&range)?;

    
    let mut iter_result =
        RangeDeserializerBuilder::with_headers(&COLUMNS)
        .from_range::<_, DataType>(&range)?;


    if let Some(result) = iter.next() {
        let row: Vec<DataType> = result?;
        //println!("{:?}", row);
        //assert_eq!(row, [DataType::from("label"), DataType::from("value")]);
    }
    /*
     else {
        return Err(From::from("expected at least three records but got none"));
    }
    
    if let Some(result) = iter.next() {
        let _row: Vec<DataType> = result?;
       // assert_eq!(row, [DataType::from("celsius"), DataType::from(22.2222)]);
    } else {
        return Err(From::from("expected at least three records but got one"));
    }*/

    Ok(())
}

fn read_db_config() {
    let filename = "db_config.toml";

    let contents = match fs::read_to_string(filename) {
        Ok(c) => c,
        Err(_) => {
            eprintln!("Could not read file `{}`", filename);
            exit(1);
        }
    };

    let _data: DbConfig = match toml::from_str(&contents) {
        Ok(d) => d,
        Err(_) => {
            eprintln!("Unable to load data from `{}`", filename);
            exit(1);
        }
    };

}

fn read_cdb_export() {
    //let path = std::path::Path::new("./CDB_export/capacitor.xlsx");

    
}

fn main() {
    //let parts_by_cdb: Vec<PartByCDB> = Vec::new();
    //let parts_by_sap: Vec<String> = Vec::new();

    //read_cdb_export();
    example().unwrap();
}

