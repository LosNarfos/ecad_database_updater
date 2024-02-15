use crate::import::sap::PartFromSAP;
use super::Part;

impl Part {
    pub fn polish_opto(&mut self, sap_part: &PartFromSAP) {
        self.opto_check_category(sap_part);
    }

    fn opto_check_category(&mut self, sap_part: &PartFromSAP) -> &mut Self{
        self.category = "OPTO".to_string();
        self
    }
}