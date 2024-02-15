use crate::import::sap::PartFromSAP;
use super::Part;

impl Part {
    pub fn polish_diode(&mut self, sap_part: &PartFromSAP) {
        self.diode_check_category(sap_part);
    }

    fn diode_check_category(&mut self, sap_part: &PartFromSAP) -> &mut Self{
        self.category = "DIO".to_string();
        self
    }
}