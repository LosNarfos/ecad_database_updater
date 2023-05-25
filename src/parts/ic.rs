use crate::import::{cdb::PartFromCDB, sap::PartFromSAP};
use super::Part;

impl Part {
    pub fn polish_ic(&mut self, cdb_part: &PartFromCDB, sap_part: &PartFromSAP) {
        self.ic_check_category(cdb_part, sap_part);
    }

    fn ic_check_category(&mut self, cdb_part: &PartFromCDB, sap_part: &PartFromSAP) -> &mut Self{
        self.category = "IC".to_string();
        self
    }
}