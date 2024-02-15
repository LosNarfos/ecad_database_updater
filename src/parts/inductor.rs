use crate::import::sap::PartFromSAP;
use super::Part;

impl Part {
    pub fn polish_inductor(&mut self, sap_part: &PartFromSAP) {
        self.inductor_check_category(sap_part);
    }

    fn inductor_check_category(&mut self, sap_part: &PartFromSAP) -> &mut Self{
        self.category = "IND".to_string();
        self
    }
}