use std::fmt;

use serde::Deserialize;

use crate::PartType;
use crate::import::sap::PartFromSAP;

pub mod capacitor;
pub mod connector;
pub mod diode;
pub mod ic;
pub mod inductor;
pub mod mechanic;
pub mod opto;
pub mod other;
pub mod resistor;
pub mod transistor;

#[derive(Clone, Deserialize, Debug)]
pub struct Part {
    #[serde(default, rename = "CDB No")]
    pub cdb_number: String,
    #[serde(default, rename = "CDB Index")]
    pub cdb_index: String,
    #[serde(default, rename = "CDB State")]
    pub cdb_state: String,
    #[serde(default, rename = "SAP No")]
    pub sap_number: String,
    #[serde(default, rename= "SAP State")]
    pub sap_state: String,
    #[serde(default, rename= "SAP Date")]
    pub sap_date: String,
    #[serde(default, rename= "Life Cycle")]     // not yet available in CDB
    pub life_cycle: String,
    #[serde(default, rename = "Altium State")]
    pub altium_state: String,
    #[serde(default, rename = "Manufacturer")]      // not yet available in CDB
    pub manufacturer: String,
    #[serde(default, rename = "Manufacturer_number")]   // not yet available in CDB
    pub manufacturer_no: String,
    #[serde(default, rename = "Second Source")]     // not yet available in CDB
    pub second_source: String,
    #[serde(default, rename = "Part Name")]
    pub part_name: String,
    #[serde(default, rename = "Value")]
    pub value: String,
    #[serde(default, rename = "Description")]
    pub description: String,
    #[serde(default, rename = "Package")]
    pub package: String,
    #[serde(default, rename = "Category")]
    pub category: String,
    #[serde(default, rename = "Type")]
    pub part_type: String,
    #[serde(default, rename = "Voltage")]
    pub voltage: String,
    #[serde(default, rename = "Current")]
    pub current: String,
    #[serde(default, rename = "Power")]
    pub power: String,
    #[serde(default, rename = "Tolerance")]
    pub tolerance: String,
    #[serde(default, rename = "Temperature")]
    pub temperature: String,
    #[serde(default, rename = "Height")]
    pub height: String,
    #[serde(default, rename = "Pins")]     // not yet available in CDB
    pub pins: String,
    #[serde(default, rename = "MTTF")]     // not yet available in CDB
    pub mttf: String,
    #[serde(default, rename = "Info1")]
    pub info1: String,
    #[serde(default, rename = "Info2")]
    pub info2: String,
    #[serde(default, rename = "Info3")]
    pub info3: String,
    #[serde(default, rename = "Info4")]
    pub info4: String,
    #[serde(default, rename = "Info5")]
    pub info5: String,
    #[serde(default, rename = "Info6")]
    pub info6: String,
    #[serde(default, rename = "Info7")]
    pub info7: String,
    #[serde(default, rename = "Info8")]
    pub info8: String,
    #[serde(default, rename = "Info9")]
    pub info9: String,
    #[serde(default, rename = "Info10")]
    pub info10: String,
    #[serde(default, rename = "Library Ref")]
    pub library_ref: String,
    #[serde(default, rename = "Library Path")]
    pub library_path: String,
    #[serde(default, rename = "Footprint Ref")]
    pub footprint_ref1: String,
    #[serde(default, rename = "Footprint Ref 2")]
    pub footprint_ref2: String,
    #[serde(default, rename = "Footprint Ref 3")]    // not yet available in CDB
    pub footprint_ref3: String,
    #[serde(default, rename = "Footprint Ref 4")]    // not yet available in CDB
    pub footprint_ref4: String,
    #[serde(default, rename = "Footprint Ref 5")]    // not yet available in CDB
    pub footprint_ref5: String,    
    #[serde(default, rename = "Footprint Path")]
    pub footprint_path1: String,
    #[serde(default, rename = "Footprint Path 2")]
    pub footprint_path2: String,
    #[serde(default, rename = "Footprint Path 3")]   // not yet available in CDB
    pub footprint_path3: String,
    #[serde(default, rename = "Footprint Path 4")]   // not yet available in CDB
    pub footprint_path4: String,
    #[serde(default, rename = "Footprint Path 5")]   // not yet available in CDB
    pub footprint_path5: String,
    #[serde(default, rename = "Model")]     // not yet available in CDB
    pub model: String,
    #[serde(default, rename = "Model Ref")] // not yet available in CDB
    pub model_ref: String,
    #[serde(default, rename = "Model Path")]    // not yet available in CDB
    pub model_path: String,
    #[serde(default, rename = "Help_URL")]   // not yet available in CDB
    pub help_url: String,
    #[serde(default, rename = "datasheet_URL")] // not yet available in CDB
    pub datasheet_url: String,
    #[serde(default, rename = "Stock 2100")]
    pub stock_2100: String,
    #[serde(default, rename = "Stock 2720")]
    pub stock_2720: String,
    #[serde(default, rename = "Price 2100")]
    pub price_2100: String,
    #[serde(default, rename = "Price 2720")]
    pub price_2720: String,
    #[serde(default, rename = "Consumption 2100")]
    pub consumption_2100: String,
    #[serde(default, rename = "Consumption 2720")]
    pub consumption_2720: String,
}


impl Part {
    
    fn check_life_cycle(&mut self, sap_part: &PartFromSAP) -> &mut Self{
        match sap_part.standard_mat.as_str() {
            "Baumer-Standardbauteil"            => self.life_cycle = "GOLD".to_string(),
            "PS-Standardbauteil"                => self.life_cycle = "WHITE".to_string(),
            "Projektbauteil"                    => self.life_cycle = "GRAY".to_string(),
            "Auslaufbauteil-keine Neudesign"    => self.life_cycle = "BLACK".to_string(),
            _                                   => self.life_cycle = "".to_string()
        }

        // log if life cycle is empty while SAP state is present (-> Part exists in SAP but has no life cycle entry)
        if sap_part.standard_mat.is_empty() && !sap_part.mat_status.is_empty() {
            crate::export::logfile::missing_life_cycle(
                sap_part.default_stock_id.as_str(),
                sap_part.mat_status.as_str(),
                self.description.as_str()
            )
        }
        self
    }

    fn check_second_source(&mut self, sap_part: &PartFromSAP) -> &mut Self{
        // checks if multiple manufacturer part numbers are given from SAP export (they are separated by "/")
        match sap_part.part_no_manufacturer.contains('/') {
            true => self.second_source = "Yes".to_string(),
            false => self.second_source = "No".to_string(),
        }
        self
    }

    fn check_manufacturer(&mut self, sap_part: &PartFromSAP) -> &mut Self{
        self.manufacturer = sap_part.manufacturer.clone();
        self
    }

    fn check_manufacturer_no(&mut self, sap_part: &PartFromSAP) -> &mut Self{
        self.manufacturer_no = sap_part.sap_mat_no_htn.clone();
        self
    }

    fn check_stock_2100(&mut self, sap_part: &PartFromSAP) -> &mut Self{
        self.stock_2100 = sap_part.stock_2100.clone();
        self
    }

    fn check_stock_2720(&mut self, sap_part: &PartFromSAP) -> &mut Self{
        self.stock_2720 = sap_part.stock_2720.clone();
        self
    }

    fn check_price(&mut self, sap_part: &PartFromSAP) -> &mut Self{

        if sap_part.price_info != "" {
            let splitted: Vec<&str> = sap_part.price_info.split(&['-', ' ', '/'][..]).collect();
            // example: 
            // let splitted: Vec<&str> = "2100-33.46 CHF/100-1,500.000".split(&['-', ' ', '/'][..]).collect();
            // assert_eq!(splitted, ["2100", "33.46", "CHF", "100", "1,500.000"]);

            let price_per_bulk: f32 = splitted[1].to_string().replace(",", "").parse().unwrap();
            let bulk_size: f32 = splitted[3].to_string().replace(",", "").parse().unwrap();
            let price_per_unit: f32 = price_per_bulk / bulk_size;
    
            self.price_2720 = format!("{:.4}", price_per_unit);
        }
        else {
            self.price_2720 = "".to_string();
        }

        self
    }

    fn check_temperature(&mut self, sap_part: &PartFromSAP) -> &mut Self{
        if !self.temperature.is_empty() { self.temperature.push('C') }
        self
    }

    fn check_height(&mut self, sap_part: &PartFromSAP) -> &mut Self{
        // convert height field to always use "mm"
        let mut base = self.height.clone();

        if base.is_empty() || base == "0.0m" {
            self.height = "".to_string();
            return self
        }

        base.truncate(base.len() - 1 );
        let mut base = base.parse::<f32>().unwrap();
        let exponent = self.height.clone().chars().last().unwrap();
    
        if exponent == 'u' {
            // height is given in µm -> convert to mm
            base = base / 1000.0;
        }
        self.height = format!("{:.2}", base) + "mm";
        self
    }

    fn check_voltage(&mut self, sap_part: &PartFromSAP) -> &mut Self{
        self.voltage = self.voltage.clone();
        if !self.voltage.is_empty() {
             self.voltage.push('V')
        }
        self
    }

    fn check_current(&mut self, sap_part: &PartFromSAP) -> &mut Self{       
        self.current = self.current.clone();
        if !self.current.is_empty() {
             self.current.push('A')
        }
        self
    }

    fn check_power(&mut self, sap_part: &PartFromSAP) -> &mut Self{
        self.power = self.power.clone();
        if !self.power.is_empty() {
            self.power.push('W')
        }
        self
    }

    fn check_tolerance(&mut self, sap_part: &PartFromSAP) -> &mut Self{
        self.tolerance = self.tolerance.clone();
        self
    }

}

pub fn polish<'a>(part_type: PartType, parts: &'a mut Vec<Part>, sap_parts: &Vec<PartFromSAP>) ->  &'a mut Vec<Part> {


    for part in parts.iter_mut() {
        
        // Search the SAP-export for a matching SAP number
        let sap_part = match sap_parts.into_iter().position(|x| x.default_stock_id == part.sap_number) {
            Some(row) => sap_parts[row].clone(),
            None => PartFromSAP::default(),
        };
        
        // modify the common fields
        part
            .check_life_cycle(&sap_part)
            .check_second_source(&sap_part)
            .check_manufacturer(&sap_part)
            .check_manufacturer_no(&sap_part)
            .check_stock_2100(&sap_part)
            .check_stock_2720(&sap_part)
            .check_price(&sap_part)
            .check_temperature(&sap_part)
            .check_height(&sap_part)
            .check_voltage(&sap_part)
            .check_current(&sap_part)
            .check_power(&sap_part)
            .check_tolerance(&sap_part);

        // modify the fields which differ between classes
        match part_type {
            PartType::Capacitor => part.polish_capacitor(&sap_part),
            PartType::Connector => part.polish_connector(&sap_part),
            PartType::Diode => part.polish_diode(&sap_part),
            PartType::Ic => part.polish_ic(&sap_part),
            PartType::Inductor => part.polish_inductor(&sap_part),
            PartType::Mechanic => part.polish_mechanic(&sap_part),
            PartType::Opto => part.polish_opto(&sap_part),
            PartType::Other => part.polish_other(&sap_part),
            PartType::Resistor => part.polish_resistor(&sap_part),
            PartType::Transistor => part.polish_transistor(&sap_part),
        }
    }
    parts
}

