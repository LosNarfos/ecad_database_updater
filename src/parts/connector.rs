use crate::import::{cdb_import::PartFromCDB, sap_import::PartFromSAP};
use super::Part;

impl Part {
    pub fn polish_connector(&mut self, cdb_part: &PartFromCDB, sap_part: &PartFromSAP) {
        // ToDo
    }
}