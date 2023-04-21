use crate::PartFromCDB;
use crate::PartFromSAP;

pub mod capacitor;

#[derive(Clone, Debug)]
pub struct Part {
    pub cdb_number: String,
    pub sap_number: String,
    pub part_name: String,
    pub value: String,
    pub package: String,
    pub temperature: String,
    pub height: String,
    pub library_ref: String,
    pub library_path: String,
    pub footprint_ref1: String,
    pub footprint_path1: String,
    pub footprint_ref2: String,
    pub footprint_path2: String,
    pub altium_state: String,
    pub description: String,
    pub voltage: String,
    pub tolerance: String,
    pub current: String,
    pub power: String,
    pub info1: String,
    pub info2: String,
    pub info3: String,
    pub info4: String,
    pub info5: String,
}
impl Part {
    fn part_check_cdb_number(&mut self, cdb_part: &PartFromCDB, sap_part: Option<&PartFromSAP>) -> &mut Self{
        self.cdb_number = cdb_part.cdb_number.clone();
        self
    }

    fn part_check_sap_number(&mut self, cdb_part: &PartFromCDB, sap_part: Option<&PartFromSAP>) -> &mut Self{
        self.sap_number = cdb_part.sap_number.clone();
        self
    }

    fn part_check_part_name(&mut self, cdb_part: &PartFromCDB, sap_part: Option<&PartFromSAP>) -> &mut Self{
        self.part_name = cdb_part.part_name.clone();
        self
    }

    fn part_check_temperature(&mut self, cdb_part: &PartFromCDB, sap_part: Option<&PartFromSAP>) -> &mut Self{
        self.temperature = cdb_part.temperature.clone() + "C";
        self
    }

    fn part_check_height(&mut self, cdb_part: &PartFromCDB, sap_part: Option<&PartFromSAP>) -> &mut Self{
        // convert height field to always use "mm"
        let mut base = cdb_part.height.clone();
        base.truncate(base.len() - 1 );
        let mut base = base.parse::<f32>().unwrap();
        let exponent = cdb_part.height.clone().chars().last().unwrap();
    
        if exponent == 'u' {
            // height is given in µm -> convert to mm
            base = base / 1000.0;
        }
        self.height = format!("{:.2}", base) + "mm";
        self
    }

}

impl Default for Part {
    fn default() -> Part {
        Part {
        cdb_number: "".to_string(),
        sap_number: "".to_string(),
        part_name: "".to_string(),
        value: "".to_string(),
        package: "".to_string(),
        temperature: "".to_string(),
        height: "".to_string(),
        library_ref: "".to_string(),
        library_path: "".to_string(),
        footprint_ref1: "".to_string(),
        footprint_path1: "".to_string(),
        footprint_ref2: "".to_string(),
        footprint_path2: "".to_string(),
        altium_state: "".to_string(),
        description: "".to_string(),
        voltage: "".to_string(),
        tolerance: "".to_string(),
        current: "".to_string(),
        power: "".to_string(),
        info1: "".to_string(),
        info2: "".to_string(),
        info3: "".to_string(),
        info4: "".to_string(),
        info5: "".to_string(),
        }
    }
}