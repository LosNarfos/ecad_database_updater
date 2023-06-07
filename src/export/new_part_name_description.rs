use crate::parts::Part;

use std::fs::OpenOptions;
use std::io::prelude::*;

fn trim_whitespace(s: &str) -> String {
    let words: Vec<_> = s.split_whitespace().collect();
    words.join(" ")
}

fn ic_adc_dac(part: &Part) -> String {
    let name = [
        part.info1.as_str(),    // Subtype
        (part.info6.clone() + " Channel").as_str(), // Number of Channels
        (part.info2.clone() + "Bit").as_str(),  // Resolution
        part.info5.as_str(),    // Interface
    ].join(" ");
    
    trim_whitespace(&name)
}

pub fn capacitor(part: &Part) -> String{
    let mut name = "".to_string();
    
    let category: String;       // always "CAP"
    let subtype: String;        // Dielectric for classic CAPs or general type for special cases (e.g. timm, EMI filters)
    let field1: String;         // capacity
    let field2: String;         // tolerance
    let field3: String;         // voltage
    let field4: String;         // Package
    let field5: String;         // max. temperature
    
    category = part.category.clone();

    subtype = if part.info1 == "" {
        part.part_type.clone()
    }
    else {
        part.info1.clone()
    };

    field1 = part.value.clone() + "F";

    field2 = if part.tolerance.contains("+-") {
        // tolerance band is equal around nominal value; Remove +-
        part.tolerance.replace("+-", "")
    }
    else {
       "".to_string()
    };

    field3 = part.voltage.clone();
    field4 = part.package.clone();
    field5 = part.temperature.split('.').last().unwrap().to_string().clone();

    // name.push_str(format!("{:>5}", part.info1).as_str()); //dielectric
    name.push_str(format!("{} {} {} {} {} {} {}", 
        category,
        subtype,
        field1,
        field2,
        field3,
        field4,
        field5,
    ).as_str());

    trim_whitespace(&name)
}

pub fn connector(part: &Part) -> String{

    let name = [
        part.category.as_str(),
        part.part_type.as_str(),
        part.info1.as_str(),
        part.info2.as_str()
        ].join(" ");

    trim_whitespace(&name)
}

pub fn diode(part: &Part) -> String{

    // replace some longer part type strings with its short abbrivations
    let part_type = match part.part_type.as_str() {
        "Standard" => "STD",
        "TVS Bidirectional" => "TVS",
        "TVS Unidirectional" => "TVS",
        "Zener" => "ZENER",
        "Schottky" => "SCHOT",
        "Rectifier" => "RECT",
        _ => part.part_type.as_str(),
    };
    
    let name = [
        part.category.as_str(),
        part_type, part.current.as_str(),
        part.power.as_str()
        ].join(" ");

    trim_whitespace(&name)
}

pub fn ic(part: &Part) -> String{
    let mut name = "".to_string();

    let ic_part_type = if part.part_type == "ADC/DAC" {
        part.info1.clone() // => ADC or DAC
    }
    else {
        part.part_type.clone()
    };


    name.push_str(format!("{:}", part.category).as_str());
    name.push_str(" ");
    name.push_str(format!("{:}", part.part_type).as_str());

    name
}

pub fn inductor(part: &Part) -> String{

    // replace some longer part type strings with its short abbrivations
    let part_type = match part.part_type.as_str() {
        "Inductor" => "Standard".to_string(),
        "Choke" => "Choke".to_string(),
        "Ferrite" => "Ferrite".to_string(),
        "Transformer" => "Trafo".to_string(),
        _ => "".to_string(), 
    };

    let mut tolerance = if part.tolerance.contains("+-") {
        part.tolerance.replace("+-", "")
    }
    else {
        part.tolerance.clone()
    };
    if part.tolerance.contains("inf") {
        tolerance = "".to_string();
    }

    let name = [
        part.category.as_str(),
        part_type.as_str(),
        (part.value.clone() + "H").as_str(),
        part.current.as_str(),
        tolerance.as_str(),
        ].join(" ");

    trim_whitespace(&name)
}

pub fn mechanic(part: &Part) -> String{
    todo!()
}

pub fn opto(part: &Part) -> String{
    todo!()
}

pub fn other(part: &Part) -> String{
    todo!()
}

pub fn resistor(part: &Part) -> String{
    todo!()
}

pub fn transistor(part: &Part) -> String{
    todo!()
}

pub fn add(part: &Part) {
    let mut file = OpenOptions::new()
    .append(true)
    .create(true)
    .open("logfiles/new_part_names.txt")
    .unwrap();

    let new_name = match part.category.as_str() {
        "CAP" => capacitor(part),
        "CON" => connector(part),
        "DIO" => diode(part),
        "IC" => ic(part),
        "IND" => inductor(part),
        "mechanic" => mechanic(part),
        "opto" => opto(part),
        "other" => other(part),
        "resistor" => resistor(part),
        "transistor" => transistor(part),
        _ => "category missmatch".to_string(),
    };


    let line = format!("{:>10}   {:<40}  {}", part.sap_number, part.description, new_name);

    if let Err(e) = writeln!(file,"{}", line) {
        eprintln!("Couldn't write to file: {}", e);
    }
}