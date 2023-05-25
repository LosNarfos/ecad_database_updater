use crate::import::{cdb::PartFromCDB, sap::PartFromSAP};
use super::Part;

impl Part {
    pub fn polish_inductor(&mut self, cdb_part: &PartFromCDB, sap_part: &PartFromSAP) {
        self.inductor_check_category(cdb_part, sap_part);
    }

    fn inductor_check_category(&mut self, cdb_part: &PartFromCDB, sap_part: &PartFromSAP) -> &mut Self{
        self.category = "IND".to_string();
        self
    }
}