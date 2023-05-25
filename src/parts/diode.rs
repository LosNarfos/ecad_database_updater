use crate::import::{cdb::PartFromCDB, sap::PartFromSAP};
use super::Part;

impl Part {
    pub fn polish_diode(&mut self, cdb_part: &PartFromCDB, sap_part: &PartFromSAP) {
        self.diode_check_category(cdb_part, sap_part);
    }

    fn diode_check_category(&mut self, cdb_part: &PartFromCDB, sap_part: &PartFromSAP) -> &mut Self{
        self.category = "DIO".to_string();
        self
    }
}