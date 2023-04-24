
use crate::import::{cdb_import::PartFromCDB, sap_import::PartFromSAP};


use super::Part;

impl Part {
    // methods specific to capacitors

    fn capacitor_check_value(&mut self, cdb_part: &PartFromCDB, sap_part: Option<&PartFromSAP>) -> &mut Self{
        self.value = cdb_part.value.clone() + "V"; // add unit to value field
        self
    }

    fn capacitor_check_package(&mut self, cdb_part: &PartFromCDB, sap_part: Option<&PartFromSAP>) -> &mut Self{
        // At the moment no package cleanup required
        self
    }
}

pub fn parts_polish_capacitor (cdb_capcitors: &Vec<PartFromCDB>, sap_capacitors: &Vec<PartFromSAP>, parts: &Vec<Part>) {

    println!("\n  Cleaning up:  Capacitor");

    for (index, cdb_capacitor) in cdb_capcitors.iter().enumerate() {
        
        let mut capacitor = Part { .. Default::default()};


        // Search the SAP-export for a matching SAP number
        let sap_part = match sap_capacitors.into_iter().position(|x| x.default_stock_id == cdb_capacitor.sap_number) {
            Some(row) => Some(&sap_capacitors[index]),
            None => None
        };

        // fill the new capacitor struct which will be send to Database.
        // Some methods are empty but can be extended if neccessary.
        capacitor
            .part_check_cdb_number(cdb_capacitor, sap_part)
            .part_check_sap_number(cdb_capacitor, sap_part)
            .part_check_part_name(cdb_capacitor, sap_part)
            .capacitor_check_value(cdb_capacitor, sap_part)
            .capacitor_check_package(cdb_capacitor, sap_part)
            .part_check_temperature(cdb_capacitor, sap_part)
            .part_check_height(cdb_capacitor, sap_part);


    }
}








