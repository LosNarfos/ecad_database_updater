//use anyhow::Error;
use odbc_api::{Error, buffers::BufferDesc, Environment, ConnectionOptions};
use crate::parts::{PartType, Part};



/// Maximum number of rows fetched with one row set. Fetching batches of rows is usually much
/// faster than fetching individual rows.
const BATCH_SIZE: usize = 5000;

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

    insert_string.push_str(part_type_to_str(part_type).as_str());
    insert_string.push_str(" (");
    
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
    insert_string.push_str("[Footprint Ref2], ");
    insert_string.push_str("[Footprint Path2], ");
    insert_string.push_str("[Footprint Ref3], ");
    insert_string.push_str("[Footprint Path3], ");
    insert_string.push_str("[Footprint Ref4], ");
    insert_string.push_str("[Footprint Path4], ");
    insert_string.push_str("[Footprint Ref5], ");
    insert_string.push_str("[Footprint Path5], ");
    insert_string.push_str("[Footprint Path5], ");
    insert_string.push_str("[Footprint Path5], ");
    insert_string.push_str("[Footprint Path5], ");
    insert_string.push_str("[Model], ");
    insert_string.push_str("[Model Ref], ");
    insert_string.push_str("[Model Path]");

    insert_string.push_str(") ");
    
    insert_string.push_str("VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)");

    insert_string



   }

pub fn insert_data(part_type: PartType, parts: Vec<Part>) -> Result<(), Error> {
    
    let env = Environment::new()?;

    let connection_string = "
        Driver={ODBC Driver 17 for SQL Server};\
        ConnSettings=SET CLIENT_ENCODING TO 'UTF8';\
        Server=SQLDBSRV11\\INST2;\
        Database=ECAD_PARTS_dev;\
        UID=ecad_user;\
        PWD=E34Corona;\
    ";
    let connection = env.connect_with_connection_string(connection_string, ConnectionOptions::default())?;
    let prepared = connection.prepare(create_insert_string(&part_type).as_str())?;

    // Truncate whole table; Out with the old, in with the new !
    let mut query = "TRUNCATE TABLE dbo.".to_string();
    query.push_str(part_type_to_str(&part_type).as_str());
    connection.execute(query.as_str(), ())?;

    // Create a columnar buffer which fits the input parameters.
    let buffer_description: [BufferDesc; 32] = [BufferDesc::Text { max_str_len: 255 }; 32];

    let capacity = parts.len();

    // Allocate memory for the array column parameters and bind it to the statement.
    let mut prebound = prepared.into_column_inserter(capacity, buffer_description)?;
    // Length of this batch
    prebound.set_num_rows(capacity);

    // Fill the buffer with values column by column
    prebound = fill_buffer_with_values(prebound,&parts);

    // buffer is filled: Send content to Database
    prebound.execute()?;

    Ok(())
}

fn fill_buffer_with_values<'a>(mut prebound: odbc_api::ColumnarBulkInserter<odbc_api::handles::StatementImpl<'a>, odbc_api::buffers::AnyBuffer>, parts: &'a Vec<Part>) 
-> odbc_api::ColumnarBulkInserter<odbc_api::handles::StatementImpl<'a>, odbc_api::buffers::AnyBuffer> {
    
    // outer loop -> iterate through the columns
    for column in (0..32) {
        
        let mut col = prebound
        .column_mut(column)
        .as_text_view()
        .expect("We know the name column to hold text.");

        //inner loop -> iterate through the rows
        for (index, part) in parts.iter().enumerate() {
            col.set_cell(index, Some(part.cdb_number.as_bytes()));
            println!("Row Index: {}", index);
        }
        println!("Column Index: {}", column);
    }
    prebound
}

// fn insert_birth_years(conn: &Connection, names: &[&str], years: &[i16]) -> Result<(), Error> {

//     // All columns must have equal length.
//     assert_eq!(names.len(), years.len());

//     let prepared = conn.prepare("INSERT INTO Birthdays (name, year) VALUES (?, ?)")?;

//     // Create a columnar buffer which fits the input parameters.
//     let buffer_description = [
//         BufferDesc::Text { max_str_len: 255 },
//         BufferDesc::I16 { nullable: false },
//     ];

//     // The capacity must be able to hold at least the largest batch. We do everything in one go, so
//     // we set it to the length of the input parameters.
//     let capacity = names.len();
//     // Allocate memory for the array column parameters and bind it to the statement.
//     let mut prebound = prepared.into_column_inserter(capacity, buffer_description)?;
//     // Length of this batch
//     prebound.set_num_rows(capacity);


//     // Fill the buffer with values column by column
//     prebound = fill_buffer_with_values(prebound,&parts);

//     prebound.execute()?;
//     Ok(())
// }


// fn main2() -> Result<(), Error> {

//    let env = Environment::new()?;
//     let connection_string = "
//         Driver={ODBC Driver 17 for SQL Server};\
//         ConnSettings=SET CLIENT_ENCODING TO 'UTF8';\
//         Server=SQLDBSRV11\\INST2;\
//         Database=ECAD_PARTS_dev;\
//         UID=ecad_user;\
//         PWD=E34Corona;\
//     ";

//     let connection = env.connect_with_connection_string(connection_string, ConnectionOptions::default())?;

//     connection.execute("TRUNCATE TABLE dbo.Birthdays", ())?;

//     let names = ["ä", "Peter", "Max"];
//     let years = [1988, 1987, 1986];
//     insert_birth_years(&connection, &names, &years).unwrap();

//     let value = 10;

//     //connection.execute("INSERT INTO [dbo].[Birthdays] ([CDB No]) VALUES (?)", ())?;

//     Ok(())
// }
