use crate::import::{cdb::PartFromCDB, sap::PartFromSAP};
use super::Part;

impl Part {
    pub fn polish_mechanic(&mut self, cdb_part: &PartFromCDB, sap_part: &PartFromSAP) {
        self.mechanic_check_category(cdb_part, sap_part);
    }

    fn mechanic_check_category(&mut self, cdb_part: &PartFromCDB, sap_part: &PartFromSAP) -> &mut Self{
        self.category = "MECH".to_string();
        self
    }
}