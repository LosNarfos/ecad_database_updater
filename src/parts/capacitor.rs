use crate::import::sap::PartFromSAP;
use super::Part;

impl Part {
    pub fn polish_capacitor(&mut self, sap_part: &PartFromSAP) {
        self.capacitor_check_value(sap_part);
        self.capacitor_check_category(sap_part);
    }

    fn capacitor_check_value(&mut self, sap_part: &PartFromSAP) -> &mut Self{
        if !self.value.is_empty() {
            self.value.push_str("F"); // add unit to value field
        }
        self
    }

    fn capacitor_check_category(&mut self, sap_part: &PartFromSAP) -> &mut Self{
        self.category = "CAP".to_string();
        self
    }

    fn capacitor_check_voltage(&mut self, sap_part: &PartFromSAP) -> &mut Self{
        if !self.voltage.is_empty() {
            self.voltage.push_str("V"); // add unit to value field
        }
        self
    }
}