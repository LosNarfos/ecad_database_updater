use crate::import::{cdb::PartFromCDB, sap::PartFromSAP};
use super::Part;

impl Part {
    pub fn polish_other(&mut self, cdb_part: &PartFromCDB, sap_part: &PartFromSAP) {
        if cdb_part.cdb_number == "A0001064625" {
            // this is the BOM Placeholder SAP number to be placed on the first schematic page
            // We replace the SAP number with "=PRJ_PCB_MatNoSAP" so when imported in Altium the real PCB Number of the current project is displayed in the
            // SAP field (instead of the old placeholder)
            self.sap_number = "=PRJ_PCB_MatNoSAP".to_string();
        }
    }
}