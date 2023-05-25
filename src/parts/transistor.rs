use crate::import::{cdb::PartFromCDB, sap::PartFromSAP};
use super::Part;

impl Part {
    pub fn polish_transistor(&mut self, cdb_part: &PartFromCDB, sap_part: &PartFromSAP) {
        self.transistor_check_category(cdb_part, sap_part);
    }

    fn transistor_check_category(&mut self, cdb_part: &PartFromCDB, sap_part: &PartFromSAP) -> &mut Self{
        self.category = "TRAN".to_string();
        self
    }
}