use crate::import::{cdb::PartFromCDB, sap::PartFromSAP};
use super::Part;

impl Part {
    pub fn polish_capacitor(&mut self, cdb_part: &PartFromCDB, sap_part: &PartFromSAP) {
        self.capacitor_check_value(cdb_part, sap_part);
        self.capacitor_check_category(cdb_part, sap_part);
    }

    fn capacitor_check_value(&mut self, cdb_part: &PartFromCDB, sap_part: &PartFromSAP) -> &mut Self{
        if cdb_part.value.is_empty() {
            self.value = cdb_part.value.clone() + "V"; // add unit to value field
        }
        self
    }

    fn capacitor_check_category(&mut self, cdb_part: &PartFromCDB, sap_part: &PartFromSAP) -> &mut Self{
        self.category = "CAP".to_string();
        self
    }
}








