use crate::import::{cdb::PartFromCDB, sap::PartFromSAP};
use super::Part;

impl Part {
    pub fn polish_resistor(&mut self, cdb_part: &PartFromCDB, sap_part: &PartFromSAP) {
        self.resistor_check_category(cdb_part, sap_part);
    }

    fn resistor_check_category(&mut self, cdb_part: &PartFromCDB, sap_part: &PartFromSAP) -> &mut Self{
        self.category = "RES".to_string();
        self
    }
}