use crate::import::sap::PartFromSAP;
use super::Part;

impl Part {
    pub fn polish_transistor(&mut self, sap_part: &PartFromSAP) {
        self.transistor_check_category(sap_part);
    }

    fn transistor_check_category(&mut self, sap_part: &PartFromSAP) -> &mut Self{
        self.category = "TRAN".to_string();
        self
    }
}