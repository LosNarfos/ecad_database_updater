#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

mod import;
mod parts;
mod database;
mod export;

use database::insert_data;
use import::import_from_cdb::import_cdb_export;
use import::import_from_sap::read_sap_export;
use odbc_api::{Environment, ConnectionOptions};
use parts::{Part,parts_polish};

use std::error::Error;

// overall container for all components.
#[derive(Debug, Default)]
pub struct Parts {
    capacitor: Vec<Part>,
    connector: Vec<Part>,
    diode: Vec<Part>,
    ic: Vec<Part>,
    inductor: Vec<Part>,
    mechanic: Vec<Part>,
    opto: Vec<Part>,
    other: Vec<Part>,
    resistor: Vec<Part>,
    transistor:Vec<Part>,
}

pub enum PartType {
    Capacitor,
    Connector,
    Diode,
    Ic,
    Inductor,
    Mechanic,
    Opto,
    Other,
    Resistor,
    Transistor
}

impl PartType {
    fn file_name_as_string(&self) -> &str {
        match self {
            PartType::Capacitor => "Capacitor",
            PartType::Connector => "Connector",
            PartType::Diode => "Diode",
            PartType::Ic =>"Ic",
            PartType::Inductor => "Inductor",
            PartType::Mechanic => "Mechanic",
            PartType::Opto => "Opto",
            PartType::Other => "Other",
            PartType::Resistor => "Resistor",
            PartType::Transistor =>"Transistor",
        }
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    
    println!("--------------------------- ECAD Database Updater ---------------------------");    
    
    // get content from CDB export
    let cdb_capacitor = import_cdb_export(PartType::Capacitor)?;
    let cdb_connector = import_cdb_export(PartType::Connector)?;
    let cdb_diode = import_cdb_export(PartType::Diode)?;
    let cdb_ic = import_cdb_export(PartType::Ic)?;
    let cdb_inductor = import_cdb_export(PartType::Inductor)?;
    let cdb_mechanic = import_cdb_export(PartType::Mechanic)?;
    let cdb_opto = import_cdb_export(PartType::Opto)?;
    let cdb_other = import_cdb_export(PartType::Other)?;
    let cdb_resistor = import_cdb_export( PartType::Resistor)?;
    let cdb_transistor = import_cdb_export(PartType::Transistor)?;
    
    println!("-----------------------------------------------------------");

    // get content from SAP export
    let sap_parts = read_sap_export()?;

    println!("-----------------------------------------------------------");

    // Modify fields
    let mut parts = Parts{.. Default::default()};
    parts.capacitor = parts_polish(PartType::Capacitor, cdb_capacitor, &sap_parts);
    parts.connector = parts_polish(PartType::Connector, cdb_connector, &sap_parts);
    parts.diode = parts_polish(PartType::Diode, cdb_diode, &sap_parts);
    parts.ic = parts_polish(PartType::Ic, cdb_ic, &sap_parts);
    parts.inductor = parts_polish(PartType::Inductor, cdb_inductor, &sap_parts);
    parts.mechanic = parts_polish(PartType::Mechanic, cdb_mechanic, &sap_parts);
    parts.opto = parts_polish(PartType::Opto, cdb_opto, &sap_parts);
    parts.other = parts_polish(PartType::Other, cdb_other, &sap_parts);
    parts.transistor = parts_polish(PartType::Transistor, cdb_transistor, &sap_parts);
    parts.resistor = parts_polish(PartType::Resistor, cdb_resistor, &sap_parts);

    let env = Environment::new()?;

    let connection_string = "\
        Driver={ODBC Driver 17 for SQL Server};\
        ConnSettings=SET CLIENT_ENCODING TO 'UTF8';\
        Server=SQLDBSRV11\\INST2;\
        Database=ECAD_PARTS_dev;\
        UID=ecad_user;\
        PWD=E34Corona;\
    ";
    let connection = env.connect_with_connection_string(connection_string, ConnectionOptions::default())?;

    insert_data(&connection, PartType::Capacitor, parts.capacitor)?;
    insert_data(&connection, PartType::Connector, parts.connector)?;
    insert_data(&connection, PartType::Diode, parts.diode)?;
    insert_data(&connection, PartType::Ic, parts.ic)?;
    insert_data(&connection, PartType::Inductor, parts.inductor)?;
    insert_data(&connection, PartType::Mechanic, parts.mechanic)?;
    insert_data(&connection, PartType::Opto, parts.opto)?;
    insert_data(&connection, PartType::Other, parts.other)?;
    insert_data(&connection, PartType::Resistor, parts.resistor)?;
    insert_data(&connection, PartType::Transistor, parts.transistor)?;

    Ok(())
}


fn main() {
    match run() {
        Ok(_) => println!(""),
        Err(error) => {
            println!("ERROR -  {}", error);

        }
    }
}

