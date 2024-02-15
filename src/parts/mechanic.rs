use crate::import::sap::PartFromSAP;
use super::Part;

impl Part {
    pub fn polish_mechanic(&mut self, sap_part: &PartFromSAP) {
        self.mechanic_check_category(sap_part);
    }

    fn mechanic_check_category(&mut self, sap_part: &PartFromSAP) -> &mut Self{
        self.category = "MECH".to_string();
        self
    }
}