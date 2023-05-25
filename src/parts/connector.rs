use crate::import::{cdb::PartFromCDB, sap::PartFromSAP};
use super::Part;

impl Part {
    pub fn polish_connector(&mut self, cdb_part: &PartFromCDB, sap_part: &PartFromSAP) {
        self.connector_check_category(cdb_part, sap_part);
    }

    fn connector_check_category(&mut self, cdb_part: &PartFromCDB, sap_part: &PartFromSAP) -> &mut Self{
        self.category = "CON".to_string();
        self
    }
}