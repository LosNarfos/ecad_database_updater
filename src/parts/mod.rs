use crate::PartType;
use crate::import::cdb::PartFromCDB;
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



#[derive(Clone, Debug, Default)]
pub struct Part {
    pub cdb_number: String,
    pub sap_number: String,
    pub part_name: String,
    pub description: String,
    pub altium_state: String,
    pub cdb_state: String,
    pub sap_state: String,
    pub life_cycle: String,
    pub manufacturer: String,
    pub manufacturer_number: String,
    pub second_source: String,
    pub stock_2100: String,
    pub stock_2720: String,
    pub price: String,
    pub category: String,
    pub part_type: String,
    pub value: String,
    pub package: String,
    pub voltage: String,
    pub current: String,
    pub power: String,
    pub tolerance: String,
    pub temperature: String,
    pub info1: String,
    pub info2: String,
    pub info3: String,
    pub info4: String,
    pub info5: String,
    pub info6: String,
    pub info7: String,
    pub info8: String,
    pub info9: String,
    pub info10: String,
    pub height: String,
    pub pins: String,
    pub mttf: String,
    pub help_url: String,
    pub datasheet_url: String,
    pub library_ref: String,
    pub library_path: String,
    pub footprint_ref1: String,
    pub footprint_path1: String,
    pub footprint_ref2: String,
    pub footprint_path2: String,
    pub footprint_ref3: String,
    pub footprint_path3: String,
    pub footprint_ref4: String,
    pub footprint_path4: String,
    pub footprint_ref5: String,
    pub footprint_path5: String,
    pub model: String,
    pub model_ref: String,
    pub model_path: String,
}

impl Part {
    
    fn copy_content(&mut self, cdb_part: &PartFromCDB, sap_part: &PartFromSAP) -> &mut Self{
        // 1-to-1 copy of the content from cdb and SAP to the new struct to be send to the database

        self.cdb_number     = cdb_part.cdb_number.clone();
        self.sap_number     = cdb_part.sap_number.clone();
        self.part_name      = cdb_part.part_name.clone();
        self.description    = cdb_part.description.clone();
        self.altium_state   = cdb_part.altium_state.clone();
        self.cdb_state      = cdb_part.cdb_state.clone();
        self.sap_state      = sap_part.mat_status.clone();
        self.life_cycle     = sap_part.standard_mat.clone();
        self.manufacturer   = cdb_part.manufacturer.clone();
        self.manufacturer_number = cdb_part.manufacturer_number.clone();
        self.second_source = "".to_string();    
        self.stock_2100     = "".to_string();
        self.stock_2720 = "".to_string();
        self.price = "".to_string();
        self.category = cdb_part.category.clone();
        self.part_type = cdb_part.part_type.clone();
        self.value = cdb_part.value.clone();
        self.package = cdb_part.package.clone();
        self.voltage = cdb_part.voltage.clone();
        self.current = cdb_part.current.clone();
        self.power = cdb_part.power.clone();
        self.tolerance = cdb_part.tolerance.clone();
        self.temperature = cdb_part.temperature.clone();
        self.info1 = cdb_part.info1.clone();
        self.info2 = cdb_part.info2.clone();
        self.info3 = cdb_part.info3.clone();
        self.info4 = cdb_part.info4.clone();
        self.info5 = cdb_part.info5.clone();
        self.info6 = cdb_part.info6.clone();
        self.info7 = cdb_part.info7.clone();
        self.info8 = cdb_part.info8.clone();
        self.info9 = cdb_part.info9.clone();
        self.info10 = cdb_part.info10.clone();
        self.height = cdb_part.height.clone();
        self.mttf = sap_part.mttf.clone();
        self.help_url = cdb_part.help_url.clone();
        self.datasheet_url = cdb_part.datasheet_url.clone();
        self.library_ref = cdb_part.library_ref.clone();
        self.library_path = cdb_part.library_path.clone();
        self.footprint_ref1 = cdb_part.footprint_ref1.clone();
        self.footprint_path1 = cdb_part.footprint_path1.clone();
        self.footprint_ref2 = cdb_part.footprint_ref2.clone();
        self.footprint_path2 = cdb_part.footprint_path2.clone();
        self.footprint_ref3 = cdb_part.footprint_ref3.clone();
        self.footprint_path3 = cdb_part.footprint_path3.clone();
        self.footprint_ref4 = cdb_part.footprint_ref4.clone();
        self.footprint_path4 = cdb_part.footprint_path4.clone();
        self.footprint_ref5 = cdb_part.footprint_ref5.clone();
        self.footprint_path5 = cdb_part.footprint_path5.clone();
        self.model = cdb_part.model.clone();
        self.model_ref = cdb_part.model_ref.clone();
        self.model_path = cdb_part.model_path.clone();

        self

    }

    fn check_life_cycle(&mut self, cdb_part: &PartFromCDB, sap_part: &PartFromSAP) -> &mut Self{
        match sap_part.standard_mat.as_str() {
            "Baumer-Standardbauteil"            => self.life_cycle = "GOLD".to_string(),
            "PS-Standardbauteil"                => self.life_cycle = "WHITE".to_string(),
            "Projektbauteil"                    => self.life_cycle = "GRAY".to_string(),
            "Auslaufbauteil-keine Neudesign"    => self.life_cycle = "BLACK".to_string(),
            _                                   => self.life_cycle = "".to_string()
        }
        self
    }

    fn check_second_source(&mut self, cdb_part: &PartFromCDB, sap_part: &PartFromSAP) -> &mut Self{
        // checks if multiple manufacturer part numbers are given from SAP export
        match sap_part.part_no_manufacturer.contains('/') {
            true => self.second_source = "Yes".to_string(),
            false => self.second_source = "No".to_string(),
        }
        self
    }

    fn check_manufacturer(&mut self, cdb_part: &PartFromCDB, sap_part: &PartFromSAP) -> &mut Self{
        self.manufacturer = sap_part.manufacturer.clone();
        self
    }

    fn check_manufacturer_no(&mut self, cdb_part: &PartFromCDB, sap_part: &PartFromSAP) -> &mut Self{
        self.manufacturer_number = sap_part.part_no_manufacturer.clone();
        self
    }

    fn check_stock_2100(&mut self, cdb_part: &PartFromCDB, sap_part: &PartFromSAP) -> &mut Self{
        self.stock_2100 = sap_part.stock_2100.clone();
        self
    }

    fn check_stock_2720(&mut self, cdb_part: &PartFromCDB, sap_part: &PartFromSAP) -> &mut Self{
        self.stock_2720 = sap_part.stock_2720.clone();
        self
    }

    fn check_price(&mut self, cdb_part: &PartFromCDB, sap_part: &PartFromSAP) -> &mut Self{

        if sap_part.price_info != "" {
            let splitted: Vec<&str> = sap_part.price_info.split(&['-', ' ', '/'][..]).collect();
            // example: 
            // let splitted: Vec<&str> = "2100-33.46 CHF/100-1,500.000".split(&['-', ' ', '/'][..]).collect();
            // assert_eq!(splitted, ["2100", "33.46", "CHF", "100", "1,500.000"]);

            let price_per_bulk: f32 = splitted[1].to_string().replace(",", "").parse().unwrap();
            let bulk_size: f32 = splitted[3].to_string().replace(",", "").parse().unwrap();
            let price_per_unit: f32 = price_per_bulk / bulk_size;
    
            self.price = format!("{:.4}", price_per_unit);
        }
        else {
            self.price = "".to_string();
        }

        self
    }

    fn check_temperature(&mut self, cdb_part: &PartFromCDB, sap_part: &PartFromSAP) -> &mut Self{
        self.temperature = cdb_part.temperature.clone();
        if !cdb_part.temperature.is_empty() { self.temperature.push('C') }
        self
    }

    fn check_height(&mut self, cdb_part: &PartFromCDB, sap_part: &PartFromSAP) -> &mut Self{
        // convert height field to always use "mm"
        let mut base = cdb_part.height.clone();

        if base.is_empty() || base == "0.0m" {
            self.height = "".to_string();
            return self
        }

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

    fn check_voltage(&mut self, cdb_part: &PartFromCDB, sap_part: &PartFromSAP) -> &mut Self{
        self.voltage = cdb_part.voltage.clone();
        if !cdb_part.voltage.is_empty() {
             self.voltage.push('V')
        }
        self
    }

    fn check_current(&mut self, cdb_part: &PartFromCDB, sap_part: &PartFromSAP) -> &mut Self{       
        self.current = cdb_part.current.clone();
        if !cdb_part.current.is_empty() {
             self.current.push('A')
        }
        self
    }

    fn check_power(&mut self, cdb_part: &PartFromCDB, sap_part: &PartFromSAP) -> &mut Self{
        self.power = cdb_part.power.clone();
        if !cdb_part.power.is_empty() {
            self.power.push('W')
        }
        self
    }

    fn check_tolerance(&mut self, cdb_part: &PartFromCDB, sap_part: &PartFromSAP) -> &mut Self{
        self.tolerance = cdb_part.tolerance.clone();
        self
    }

}

pub fn polish(part_type: PartType, cdb_parts: Vec<PartFromCDB>, sap_parts: &Vec<PartFromSAP>) ->  Vec<Part> {

    let mut parts: Vec<Part> = Vec::new();

    for cdb_part in cdb_parts.iter() {
        
        let mut part = Part { .. Default::default() };

        // Search the SAP-export for a matching SAP number
        let sap_part = match sap_parts.into_iter().position(|x| x.default_stock_id == cdb_part.sap_number) {
            Some(row) => sap_parts[row].clone(),
            None => PartFromSAP::default(),
        };

        // copy all fields to new part struct which represents the database
        part.copy_content(cdb_part, &sap_part);

        // modify the common fields
        part
            .check_life_cycle(cdb_part, &sap_part)
            .check_second_source(cdb_part, &sap_part)
            .check_stock_2100(cdb_part, &sap_part)
            .check_stock_2720(cdb_part, &sap_part)
            .check_price(cdb_part, &sap_part)
            .check_temperature(cdb_part, &sap_part)
            .check_height(cdb_part, &sap_part)
            .check_voltage(cdb_part, &sap_part)
            .check_current(cdb_part, &sap_part)
            .check_power(cdb_part, &sap_part)
            .check_tolerance(cdb_part, &sap_part);

        // modify the fields which differ between classes
        match part_type {
            PartType::Capacitor => part.polish_capacitor(cdb_part, &sap_part),
            PartType::Connector => part.polish_connector(cdb_part, &sap_part),
            PartType::Diode => part.polish_diode(cdb_part, &sap_part),
            PartType::Ic => part.polish_ic(cdb_part, &sap_part),
            PartType::Inductor => part.polish_inductor(cdb_part, &sap_part),
            PartType::Mechanic => part.polish_mechanic(cdb_part, &sap_part),
            PartType::Opto => part.polish_opto(cdb_part, &sap_part),
            PartType::Other => part.polish_other(cdb_part, &sap_part),
            PartType::Resistor => part.polish_resistor(cdb_part, &sap_part),
            PartType::Transistor => part.polish_transistor(cdb_part, &sap_part),
        }
        parts.push(part);
    }
    parts
}

