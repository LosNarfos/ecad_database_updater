
use calamine::{open_workbook, Error, Xlsx, Reader, RangeDeserializerBuilder};

use crate::{parts::Part, PartType};


fn part_type_to_str(part_type: &PartType) -> String {

    let part_type_as_string = match part_type {
        PartType::Capacitor => "Capacitor",
        PartType::Connector => "Connector",
        PartType::Diode => "Diode",
        PartType::Ic =>"Ic",
        PartType::Inductor => "Inductor",
        PartType::Mechanic => "Mechanic",
        PartType::Opto => "Opto",
        PartType::Other => "Other",
        PartType::Resistor => "Resistor",
        PartType::Transistor =>"Transistor",
    };
    part_type_as_string.to_string()
}


pub fn import(part_class: PartType) -> Result<Vec<Part>, Error> {
    let mut parts: Vec<Part> = Vec::new();

    // open Excel file
    // let mut output_file = File::create("SAP_Export\\Extract_SAP4Zuken_fixed.csv")?;
    //println!("  Updating table in database: {}", part_type_to_str(&part_type));

    let path = format!("{}.xlsx", part_class.to_string());
    //let path = format!("CDB_Export\\{}.xlsx", part_class.to_string());

    println!("  {}", path);
    let mut workbook: Xlsx<_> = open_workbook(path)?;

    // open sheet in Excel file
    let range = workbook
        .worksheet_range("cdb_export")
        .ok_or(calamine::Error::Msg("Cannot find data worksheet"))??;

    let iter_result = RangeDeserializerBuilder::new().from_range(&range)?;
    
    // write to parts vector
    for (index, row) in iter_result.enumerate() {
        match row {
            Ok(row) => {
                parts.push(row);
            }
            Err(row) => println!("{}: {:?}", index + 2, row),
        }
    }
    Ok(parts)

}