use crate::import::{import_from_cdb::PartFromCDB, import_from_sap::PartFromSAP};
use super::Part;

impl Part {
    pub fn polish_connector(&mut self, cdb_part: &PartFromCDB, sap_part: &PartFromSAP) {
        // ToDo
    }
}