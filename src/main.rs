#![allow(dead_code)]
#![allow(unused_variables)]

mod import;
mod parts;

use import::cdb_import::read_cdb_export;
use import::sap_import::read_sap_export;
use parts::capacitor::parts_polish_capacitor;
use parts::connector::parts_polish_connector;
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
    let cdb_optor = read_cdb_export("Opto")?;
    let cdb_other = read_cdb_export("Other")?;
    let cdb_resistor = read_cdb_export( "Resistor")?;
    let cdb_transistor = read_cdb_export("Transistor")?;

    // polish cdb data for better readability in Altium
    let capacitor = parts_polish_capacitor(&cdb_capacitor, &sap_parts);
    let connector = parts_polish_connector(&cdb_connector,&sap_parts);


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

