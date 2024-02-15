use crate::import::sap::PartFromSAP;
use super::Part;

impl Part {
    pub fn polish_resistor(&mut self, sap_part: &PartFromSAP) {
        self.resistor_check_category(sap_part);
    }

    fn resistor_check_category(&mut self, sap_part: &PartFromSAP) -> &mut Self{
        self.category = "RES".to_string();
        self
    }
}