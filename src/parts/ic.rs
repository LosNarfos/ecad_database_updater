use crate::import::sap::PartFromSAP;
use super::Part;

impl Part {
    pub fn polish_ic(&mut self, sap_part: &PartFromSAP) {
        self.ic_check_category(sap_part);
    }

    fn ic_check_category(&mut self, sap_part: &PartFromSAP) -> &mut Self{
        self.category = "IC".to_string();
        self
    }
}