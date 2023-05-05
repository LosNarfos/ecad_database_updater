use std::borrow::Borrow;

use odbc_api::{Error, buffers::BufferDesc, Environment, ConnectionOptions, Connection};
use crate::{parts::{Part}, PartType};



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

fn create_insert_string(part_type: &PartType) -> String {

    let mut insert_string = "INSERT INTO ".to_string();
    insert_string.push_str("[dbo].[");
    insert_string.push_str(part_type_to_str(part_type).as_str());
    insert_string.push_str("] (");
    
    insert_string.push_str("[CDB No], ");
    insert_string.push_str("[SAP No], ");
    insert_string.push_str("[Part Name], ");
    insert_string.push_str("[Description], ");
    insert_string.push_str("[Altium State], ");
    insert_string.push_str("[CDB State], ");
    insert_string.push_str("[SAP State], ");
    insert_string.push_str("[Life Cycle], ");
    insert_string.push_str("[Manufacturer], ");
    insert_string.push_str("[Manufacturer No], ");
    insert_string.push_str("[Second Source], ");
    insert_string.push_str("[Stock 2100], ");
    insert_string.push_str("[Stock 2720], ");
    insert_string.push_str("[Price], ");
    insert_string.push_str("[Category], ");
    insert_string.push_str("[Type], ");
    insert_string.push_str("[Value], ");
    insert_string.push_str("[Package], ");
    insert_string.push_str("[Voltage], ");
    insert_string.push_str("[Current], ");
    insert_string.push_str("[Power], ");
    insert_string.push_str("[Tolerance], ");
    insert_string.push_str("[Temperature], ");
    insert_string.push_str("[Info1], ");
    insert_string.push_str("[Info2], ");
    insert_string.push_str("[Info3], ");
    insert_string.push_str("[Info4], ");
    insert_string.push_str("[Info5], ");
    insert_string.push_str("[Info6], ");
    insert_string.push_str("[Info7], ");
    insert_string.push_str("[Info8], ");
    insert_string.push_str("[Info9], ");
    insert_string.push_str("[Info10], ");
    insert_string.push_str("[Height], ");
    insert_string.push_str("[Pins], ");
    insert_string.push_str("[MTTF], ");
    insert_string.push_str("[HelpURL], ");
    insert_string.push_str("[DatasheetURL], ");
    insert_string.push_str("[Library Ref], ");
    insert_string.push_str("[Library Path], ");
    insert_string.push_str("[Footprint Ref], ");
    insert_string.push_str("[Footprint Path], ");
    insert_string.push_str("[Footprint Ref 2], ");
    insert_string.push_str("[Footprint Path 2], ");
    insert_string.push_str("[Footprint Ref 3], ");
    insert_string.push_str("[Footprint Path 3], ");
    insert_string.push_str("[Footprint Ref 4], ");
    insert_string.push_str("[Footprint Path 4], ");
    insert_string.push_str("[Footprint Ref 5], ");
    insert_string.push_str("[Footprint Path 5], ");
    insert_string.push_str("[Model], ");
    insert_string.push_str("[Model Ref], ");
    insert_string.push_str("[Model Path] ");

    insert_string.push_str(") VALUES (");
    insert_string.push_str("?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?");
    insert_string.push_str(")");

    insert_string



   }

fn parts_to_columnar_bulk(parts: Vec<Part>) -> Vec<Vec<String>> {
    
    let mut all_columns: Vec<Vec<String>> = Vec::new();
    
    let mut cdb_number: Vec<String> = Vec::new();
    let mut sap_number: Vec<String> = Vec::new();
    let mut part_name: Vec<String> = Vec::new();
    let mut description: Vec<String> = Vec::new();
    let mut altium_state: Vec<String> = Vec::new();
    let mut cdb_state: Vec<String> = Vec::new();
    let mut sap_state: Vec<String> = Vec::new();
    let mut life_cycle: Vec<String> = Vec::new();
    let mut manufacturer: Vec<String> = Vec::new();
    let mut manufacturer_number: Vec<String> = Vec::new();
    let mut second_source: Vec<String> = Vec::new();
    let mut stock_2100: Vec<String> = Vec::new();
    let mut stock_2720: Vec<String> = Vec::new();
    let mut price: Vec<String> = Vec::new();
    let mut category: Vec<String> = Vec::new();
    let mut part_type: Vec<String> = Vec::new();
    let mut value: Vec<String> = Vec::new();
    let mut package: Vec<String> = Vec::new();
    let mut voltage: Vec<String> = Vec::new();
    let mut current: Vec<String> = Vec::new();
    let mut power: Vec<String> = Vec::new();
    let mut tolerance: Vec<String> = Vec::new();
    let mut temperature: Vec<String> = Vec::new();
    let mut info1: Vec<String> = Vec::new();
    let mut info2: Vec<String> = Vec::new();
    let mut info3: Vec<String> = Vec::new();
    let mut info4: Vec<String> = Vec::new();
    let mut info5: Vec<String> = Vec::new();
    let mut info6: Vec<String> = Vec::new();
    let mut info7: Vec<String> = Vec::new();
    let mut info8: Vec<String> = Vec::new();
    let mut info9: Vec<String> = Vec::new();
    let mut info10: Vec<String> = Vec::new();
    let mut height: Vec<String> = Vec::new();
    let mut pins: Vec<String> = Vec::new();
    let mut mttf: Vec<String> = Vec::new();
    let mut help_url: Vec<String> = Vec::new();
    let mut datasheet_url: Vec<String> = Vec::new();
    let mut library_ref: Vec<String> = Vec::new();
    let mut library_path: Vec<String> = Vec::new();
    let mut footprint_ref1: Vec<String> = Vec::new();
    let mut footprint_path1: Vec<String> = Vec::new();
    let mut footprint_ref2: Vec<String> = Vec::new();
    let mut footprint_path2: Vec<String> = Vec::new();
    let mut footprint_ref3: Vec<String> = Vec::new();
    let mut footprint_path3: Vec<String> = Vec::new();
    let mut footprint_ref4: Vec<String> = Vec::new();
    let mut footprint_path4: Vec<String> = Vec::new();
    let mut footprint_ref5: Vec<String> = Vec::new();
    let mut footprint_path5: Vec<String> = Vec::new();
    let mut model: Vec<String> = Vec::new();
    let mut model_ref: Vec<String> = Vec::new();
    let mut model_path: Vec<String> = Vec::new();


    for (index, part) in parts.iter().enumerate() {
        cdb_number.push(part.cdb_number.clone());
        sap_number.push(part.sap_number.clone());
        part_name.push(part.part_name.clone());
        description.push(part.description.clone());
        altium_state.push(part.altium_state.clone());
        cdb_state.push(part.cdb_state.clone());
        sap_state.push(part.sap_state.clone());
        life_cycle.push(part.life_cycle.clone());
        manufacturer.push(part.manufacturer.clone());
        manufacturer_number.push(part.manufacturer_number.clone());
        second_source.push(part.second_source.clone());
        stock_2100.push(part.stock_2100.clone());
        stock_2720.push(part.stock_2720.clone());
        price.push(part.price.clone());
        category.push(part.category.clone());
        part_type.push(part.part_type.clone());
        value.push(part.value.clone());
        package.push(part.package.clone());
        voltage.push(part.voltage.clone());
        current.push(part.current.clone());
        power.push(part.power.clone());
        tolerance.push(part.tolerance.clone());
        temperature.push(part.temperature.clone());
        info1.push(part.info1.clone());
        info2.push(part.info2.clone());
        info3.push(part.info3.clone());
        info4.push(part.info4.clone());
        info5.push(part.info5.clone());
        info6.push(part.info6.clone());
        info7.push(part.info7.clone());
        info8.push(part.info8.clone());
        info9.push(part.info9.clone());
        info10.push(part.info10.clone());
        height.push(part.height.clone());
        pins.push(part.pins.clone());
        mttf.push(part.mttf.clone());
        help_url.push(part.help_url.clone());
        datasheet_url.push(part.datasheet_url.clone());
        library_ref.push(part.library_ref.clone());
        library_path.push(part.library_path.clone());
        footprint_ref1.push(part.footprint_ref1.clone());
        footprint_path1.push(part.footprint_path1.clone());
        footprint_ref2.push(part.footprint_ref2.clone());
        footprint_path2.push(part.footprint_path2.clone());
        footprint_ref3.push(part.footprint_ref3.clone());
        footprint_path3.push(part.footprint_path3.clone());
        footprint_ref4.push(part.footprint_ref4.clone());
        footprint_path4.push(part.footprint_path4.clone());
        footprint_ref5.push(part.footprint_ref5.clone());
        footprint_path5.push(part.footprint_path5.clone());
        model.push(part.model.clone());
        model_ref.push(part.model_ref.clone());
        model_path.push(part.model_path.clone());
    }
    
    all_columns.push(cdb_number);
    all_columns.push(sap_number);
    all_columns.push(part_name);
    all_columns.push(description);
    all_columns.push(altium_state);
    all_columns.push(cdb_state);
    all_columns.push(sap_state);
    all_columns.push(life_cycle);
    all_columns.push(manufacturer);
    all_columns.push(manufacturer_number);
    all_columns.push(second_source);
    all_columns.push(stock_2100);
    all_columns.push(stock_2720);
    all_columns.push(price);
    all_columns.push(category);
    all_columns.push(part_type);
    all_columns.push(value);
    all_columns.push(package);
    all_columns.push(voltage);
    all_columns.push(current);
    all_columns.push(power);
    all_columns.push(tolerance);
    all_columns.push(temperature);
    all_columns.push(info1);
    all_columns.push(info2);
    all_columns.push(info3);
    all_columns.push(info4);
    all_columns.push(info5);
    all_columns.push(info6);
    all_columns.push(info7);
    all_columns.push(info8);
    all_columns.push(info9);
    all_columns.push(info10);
    all_columns.push(height);
    all_columns.push(pins);
    all_columns.push(mttf);
    all_columns.push(help_url);
    all_columns.push(datasheet_url);
    all_columns.push(library_ref);
    all_columns.push(library_path);
    all_columns.push(footprint_ref1);
    all_columns.push(footprint_path1);
    all_columns.push(footprint_ref2);
    all_columns.push(footprint_path2);
    all_columns.push(footprint_ref3);
    all_columns.push(footprint_path3);
    all_columns.push(footprint_ref4);
    all_columns.push(footprint_path4);
    all_columns.push(footprint_ref5);
    all_columns.push(footprint_path5);
    all_columns.push(model);
    all_columns.push(model_ref);
    all_columns.push(model_path);

    all_columns
}

pub fn insert_data(connection: &Connection, part_type: PartType, parts: Vec<Part>) -> Result<(), Error> {
    

    println!("  Updating table in database: {}", part_type_to_str(&part_type));

    // let env = Environment::new()?;

    // let connection_string = "\
    //     Driver={ODBC Driver 17 for SQL Server};\
    //     ConnSettings=SET CLIENT_ENCODING TO 'UTF8';\
    //     Server=SQLDBSRV11\\INST2;\
    //     Database=ECAD_PARTS_dev;\
    //     UID=ecad_user;\
    //     PWD=E34Corona;\
    // ";
    // let connection = env.connect_with_connection_string(connection_string, ConnectionOptions::default())?;

    // Truncate whole table; Out with the old, in with the new !
    let mut query = "TRUNCATE TABLE [dbo].[".to_string();
    query.push_str(part_type_to_str(&part_type).as_str());
    query.push_str("]");
    connection.execute(query.as_str(), ())?;

    let parts = parts_to_columnar_bulk(parts);

    // Create a columnar buffer which fits the input parameters.
    let buffer_description: [BufferDesc; 53] = [BufferDesc::Text { max_str_len: 255 }; 53];
    let capacity = parts[0].len();

    // Allocate memory for the array column parameters and bind it to the statement.
    let query_string = create_insert_string(&part_type);

    let prepared = connection.prepare(query_string.borrow())?;
    let mut prebound = prepared.into_column_inserter(capacity, buffer_description)?;
    // Length of this batch
    prebound.set_num_rows(capacity);

    // Fill the buffer with values column by column
    for (index, column) in parts.iter().enumerate() {

        let mut cell = prebound
            .column_mut(index)
            .as_text_view()
            .expect("We know the name column to hold text.");

        for (index2, entry) in column.iter().enumerate() {
            cell.set_cell(index2, Some(entry.as_bytes()));
        }
    }

    // buffer is filled: Send content to Database
    prebound.execute()?;

    Ok(())
}