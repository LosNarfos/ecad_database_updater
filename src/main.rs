#![allow(dead_code)]
#![allow(unused_variables)]
//#![allow(unused_imports)]

// Path for SAP
// \\baumernet.org\ch01apps\sap\2100_ep1\cad\sap-zuken\

mod import;
mod export;
mod parts;

use std::{error::Error, fmt};
use odbc_api::Environment;
use parts::Part;


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

impl fmt::Display for PartType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PartType::Capacitor => write!(f, "Capacitor"),
            PartType::Connector => write!(f, "Connector"),
            PartType::Diode => write!(f, "Diode"),
            PartType::Ic => write!(f, "Ic"),
            PartType::Inductor => write!(f, "Inductor"),
            PartType::Mechanic => write!(f, "Mechanic"),
            PartType::Opto => write!(f, "Opto"),
            PartType::Other => write!(f, "Other"),
            PartType::Resistor => write!(f, "Resistor"),
            PartType::Transistor => write!(f, "Transistor"),
        }
    }
}


fn run() -> Result<(), Box<dyn Error>> {
    
    println!("--------------------------- ECAD Database Updater ---------------------------");
   
    export::logfile::delete_logfiles();

    println!("Reading in CDB-Excel Files..");
    
    let mut capacitor = import::cdb::import(PartType::Capacitor)?;
    let mut connector = import::cdb::import(PartType::Connector)?;
    let mut diode = import::cdb::import(PartType::Diode)?;
    let mut ic = import::cdb::import(PartType::Ic)?;
    let mut inductor = import::cdb::import(PartType::Inductor)?;
    let mut mechanic = import::cdb::import(PartType::Mechanic)?;
    let mut opto = import::cdb::import(PartType::Opto)?;
    let mut other = import::cdb::import(PartType::Other)?;
    let mut resistor = import::cdb::import(PartType::Resistor)?;
    let mut transistor = import::cdb::import(PartType::Transistor)?;
    
    println!("Reading in SAP-CSV File..");
    let sap_zuken_exchange_dir = "\\\\baumernet.org\\ch01apps\\SAP\\2100_ep1\\cad\\SAP-ZUKEN";
    let sap_to_zuken_filename = "Extract_SAP4Zuken.csv";
    let zuken_to_sap_filename = "Extract_Zuken4SAP.csv";

    println!("  dAAAAAAAAAAAd     {}", capacitor[0].cdb_state);

    // println!("  Copy SAP file from {}\\{}", sap_zuken_exchange_dir, sap_to_zuken_filename);
    // fs::copy(
    //     sap_zuken_exchange_dir.to_owned() + "\\" + sap_to_zuken_filename,
    //     "SAP_Export\\".to_owned() + sap_to_zuken_filename)
    //     .unwrap();
    println!("  Import SAP file..");
    let sap_parts = import::sap::import()?;

    println!("Polishing parameters..");
    let capacitor = parts::polish(PartType::Capacitor, &mut capacitor, &sap_parts);
    let connector = parts::polish(PartType::Connector, &mut connector, &sap_parts);
    let diode = parts::polish(PartType::Diode, &mut diode, &sap_parts);
    let ic = parts::polish(PartType::Ic, &mut ic, &sap_parts);
    let inductor = parts::polish(PartType::Inductor, &mut inductor, &sap_parts);
    let mechanic = parts::polish(PartType::Mechanic, &mut mechanic, &sap_parts);
    let opto = parts::polish(PartType::Opto, &mut opto, &sap_parts);
    let other = parts::polish(PartType::Other, &mut other, &sap_parts);
    let resistor = parts::polish(PartType::Resistor, &mut resistor, &sap_parts);
    let transistor = parts::polish(PartType::Transistor, &mut transistor, &sap_parts);

    //println!("Create new SAP-File");
    //export::sap::export(&parts);

    // println!("  Copying file to {}\\{}", sap_zuken_exchange_dir, zuken_to_sap_filename);
    // fs::copy(
    //     "SAP_Export\\".to_owned() + zuken_to_sap_filename,
    //     sap_zuken_exchange_dir.to_owned() + "\\" + zuken_to_sap_filename)
    //     .unwrap();

    println!("Updating tables in database..");
    let env = Environment::new()?;
    let connection = export::database::connect(&env)?;

    export::database::insert(&connection, PartType::Capacitor, capacitor)?;
    export::database::insert(&connection, PartType::Connector, connector)?;
    export::database::insert(&connection, PartType::Diode, diode)?;
    export::database::insert(&connection, PartType::Ic, ic)?;
    export::database::insert(&connection, PartType::Inductor, inductor)?;
    export::database::insert(&connection, PartType::Mechanic, mechanic)?;
    export::database::insert(&connection, PartType::Opto, opto)?;
    export::database::insert(&connection, PartType::Other, other)?;
    export::database::insert(&connection, PartType::Resistor, resistor)?;
    export::database::insert(&connection, PartType::Transistor, transistor)?;

    println!("");
    println!("--------------------------- Finished ---------------------------");
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

