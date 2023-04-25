#![allow(dead_code)]
#![allow(unused_variables)]

mod import;
mod parts;
mod database;


use database::insert_data;
use import::cdb_import::read_cdb_export;
use import::sap_import::read_sap_export;
use parts::{parts_polish, PartType};

use std::error::Error;


fn run() -> Result<(), Box<dyn Error>> {

    // get content from SAP export
    let sap_parts = read_sap_export()?;

    // get content from CDB export
    let cdb_capacitor = read_cdb_export("Capacitor")?;
    let cdb_connector = read_cdb_export("Connector")?;
    let cdb_diode = read_cdb_export("Diode")?;
    let cdb_ic = read_cdb_export("IC")?;
    let cdb_inductor = read_cdb_export("Inductor")?;
    let cdb_mechanic = read_cdb_export("Mechanic")?;
    let cdb_opto = read_cdb_export("Opto")?;
    let cdb_other = read_cdb_export("Other")?;
    let cdb_resistor = read_cdb_export( "Resistor")?;
    let cdb_transistor = read_cdb_export("Transistor")?;

    // Modify fields
    let capacitor = parts_polish(PartType::Capacitor, cdb_capacitor, &sap_parts);
    let connector = parts_polish(PartType::Connector, cdb_connector, &sap_parts);
    // let diode = parts_polish(PartType::Diode, cdb_diode, &sap_parts);
    // let ic = parts_polish(PartType::Ic, cdb_ic, &sap_parts);
    // let inductor = parts_polish(PartType::Inductor, cdb_inductor, &sap_parts);
    // let mechanic = parts_polish(PartType::Mechanic, cdb_mechanic, &sap_parts);
    // let opto = parts_polish(PartType::Opto, cdb_opto, &sap_parts);
    // let other = parts_polish(PartType::Other, cdb_other, &sap_parts);
    // let transistor = parts_polish(PartType::Transistor, cdb_transistor, &sap_parts);
    // let resistor = parts_polish(PartType::Resistor, cdb_resistor, &sap_parts);

    // send to database
    insert_data(PartType::Capacitor, capacitor)?;

    Ok(())
}


fn main() {
    println!("--------------------------- ECAD Database Updater ---------------------------\n");    
    match run() {
        Ok(_) => println!(""),
        Err(error) => {
            println!("ERROR -  {}", error);

        }
    }
}

