use crate::import::{cdb_import::PartFromCDB, sap_import::PartFromSAP};
use super::Part;

impl Part {
    // methods specific to capacitors
}

pub fn parts_polish_connector(cdb_parts: &Vec<PartFromCDB>, sap_parts: &Vec<PartFromSAP>) ->  Vec<Part> {

    println!("\n  Cleaning up:  Connector");
    let mut parts: Vec<Part> = Vec::new();

    for (index, cdb_part) in cdb_parts.iter().enumerate() {
        
        let mut part = Part { .. Default::default() };

        // Search the SAP-export for a matching SAP number
        let sap_part = match sap_parts.into_iter().position(|x| x.default_stock_id == cdb_part.sap_number) {
            Some(row) => sap_parts[row].clone(),
            None => PartFromSAP::default(),
        };

        // fill the new capacitor struct which will be send to Database.
        part
            .copy_content(cdb_part, &sap_part)
            .check_price(cdb_part, &sap_part)
            .check_life_cycle(cdb_part, &sap_part)
            .check_second_source(cdb_part, &sap_part)
            .check_stock_2100(cdb_part, &sap_part)
            .check_stock_2720(cdb_part, &sap_part)
            .check_voltage(cdb_part, &sap_part)
            .check_current(cdb_part, &sap_part)
            .check_power(cdb_part, &sap_part)
            .check_tolerance(cdb_part, &sap_part)
            .check_temperature(cdb_part, &sap_part)
            .check_price(cdb_part, &sap_part)
            .check_temperature(cdb_part, &sap_part)
            .check_height(cdb_part, &sap_part);

        parts.push(part);
    }
    parts
}
