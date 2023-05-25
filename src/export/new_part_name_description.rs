use crate::parts::Part;

use std::fs::OpenOptions;
use std::io::prelude::*;

pub fn capacitor(part: &Part) -> String{
    let mut name = "".to_string();

    //let test = format!("{:>5}", part.part_type);
    name.push_str(format!("{:>4}", part.category).as_str());
    name.push_str(" ");
    name.push_str(format!("{:>5}", part.part_type).as_str());
    name.push_str(" ");
    name.push_str(format!("{:>4}", part.value).as_str());
    name.push_str(" ");
    name.push_str(format!("{:>4}", part.voltage).as_str());
    name.push_str(" ");
    name.push_str(format!("{:>5}", part.package).as_str());
    name.push_str(" ");
    name.push_str(format!("{:>5}", part.temperature).as_str());

    name
}

pub fn connector(part: &Part) -> String{
    todo!()
}

pub fn diode(part: &Part) -> String{
    todo!()
}

pub fn ic(part: &Part) -> String{
    todo!()
}

pub fn inductor(part: &Part) -> String{
    todo!()
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
        "connector" => connector(part),
        "diode" => diode(part),
        "ic" => ic(part),
        "inductor" => inductor(part),
        "mechanic" => mechanic(part),
        "opto" => opto(part),
        "other" => other(part),
        "resistor" => resistor(part),
        "transistor" => transistor(part),
        _ => "category missmatch".to_string(),
    };


    let line = format!("{:>8}   {:<40}  {:<40}", part.sap_number, part.description, new_name);

    if let Err(e) = writeln!(file,"{}", line) {
        eprintln!("Couldn't write to file: {}", e);
    }
}