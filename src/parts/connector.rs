use crate::import::sap::PartFromSAP;
use super::Part;

impl Part {
    pub fn polish_connector(&mut self, sap_part: &PartFromSAP) {
        self.connector_check_category(sap_part);
        self.connector_check_info2(sap_part);
    }

    fn connector_check_category(&mut self, sap_part: &PartFromSAP) -> &mut Self{
        self.category = "CON".to_string();
        self
    }

    fn connector_check_info2(&mut self, sap_part: &PartFromSAP) -> &mut Self{
        if self.info2.len() > 2 {
            self.info2 = self.info2[..self.info2.len()-2].to_string() + "pos";
        }
        self
    }
}