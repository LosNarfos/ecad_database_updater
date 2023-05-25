use crate::import::{cdb::PartFromCDB, sap::PartFromSAP};
use super::Part;

impl Part {
    pub fn polish_opto(&mut self, cdb_part: &PartFromCDB, sap_part: &PartFromSAP) {
        self.opto_check_category(cdb_part, sap_part);
    }

    fn opto_check_category(&mut self, cdb_part: &PartFromCDB, sap_part: &PartFromSAP) -> &mut Self{
        self.category = "OPTO".to_string();
        self
    }
}