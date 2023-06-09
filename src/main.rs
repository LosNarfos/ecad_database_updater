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
use std::fs;


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
    
    // println!("--------------------------- ECAD Database Updater ---------------------------");
   
    export::logfile::delete_logfiles();

    println!("Reading in CDB-Excel Files..");
    let cdb_capacitor = import::cdb::import(PartType::Capacitor)?;
    let cdb_connector = import::cdb::import(PartType::Connector)?;
    let cdb_diode = import::cdb::import(PartType::Diode)?;
    let cdb_ic = import::cdb::import(PartType::Ic)?;
    let cdb_inductor = import::cdb::import(PartType::Inductor)?;
    let cdb_mechanic = import::cdb::import(PartType::Mechanic)?;
    let cdb_opto = import::cdb::import(PartType::Opto)?;
    let cdb_other = import::cdb::import(PartType::Other)?;
    let cdb_resistor = import::cdb::import(PartType::Resistor)?;
    let cdb_transistor = import::cdb::import(PartType::Transistor)?;
    
    println!("Reading in SAP-CSV File..");
    let sap_parts = import::sap::import()?;

    println!("Polishing parameters..");
    let mut parts = Parts{.. Default::default()};
    parts.capacitor = parts::polish(PartType::Capacitor, cdb_capacitor, &sap_parts);
    parts.connector = parts::polish(PartType::Connector, cdb_connector, &sap_parts);
    parts.diode = parts::polish(PartType::Diode, cdb_diode, &sap_parts);
    parts.ic = parts::polish(PartType::Ic, cdb_ic, &sap_parts);
    parts.inductor = parts::polish(PartType::Inductor, cdb_inductor, &sap_parts);
    parts.mechanic = parts::polish(PartType::Mechanic, cdb_mechanic, &sap_parts);
    parts.opto = parts::polish(PartType::Opto, cdb_opto, &sap_parts);
    parts.other = parts::polish(PartType::Other, cdb_other, &sap_parts);
    parts.transistor = parts::polish(PartType::Transistor, cdb_transistor, &sap_parts);
    parts.resistor = parts::polish(PartType::Resistor, cdb_resistor, &sap_parts);


    let sap_zuken_exchange_dir = "\\\\baumernet.org\\ch01apps\\SAP\\100_ep1\\cad\\SAP-ZUKEN";

    "\\\\baumernet.org\ch01apps\SAP\2100_ep1\cad\SAP-ZUKEN"
   "\\\\baumernet.org\\ch01apps\\SAP\\100_ep1\\cad\\SAP-ZUKEN"

    println!("Get SAP file from {}", sap_zuken_exchange_dir);
    fs::copy(sap_zuken_exchange_dir.to_owned() + "Extract_SAP4Zuken.csv", "SAP_Export\\Extract_Zuken4SAP.csv").unwrap();
    println!("Create new SAP file..");
    export::sap::export(&parts);
    println!("Copying new SAP file to {}\\Extract_Zuken4SAP.csv", sap_zuken_exchange_dir);
    fs::copy("SAP_Export\\Extract_Zuken4SAP.csv", sap_zuken_exchange_dir.to_owned() + "\\Extract_Zuken4SAP.csv").unwrap();

    println!("Updating tables in database..");
    let env = Environment::new()?;
    let connection = export::database::connect(&env)?;

    // export::database::insert(&connection, PartType::Capacitor, parts.capacitor)?;
    // export::database::insert(&connection, PartType::Connector, parts.connector)?;
    // export::database::insert(&connection, PartType::Diode, parts.diode)?;
    // export::database::insert(&connection, PartType::Ic, parts.ic)?;
    // export::database::insert(&connection, PartType::Inductor, parts.inductor)?;
    // export::database::insert(&connection, PartType::Mechanic, parts.mechanic)?;
    // export::database::insert(&connection, PartType::Opto, parts.opto)?;
    // export::database::insert(&connection, PartType::Other, parts.other)?;
    // export::database::insert(&connection, PartType::Resistor, parts.resistor)?;
    // export::database::insert(&connection, PartType::Transistor, parts.transistor)?;

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

