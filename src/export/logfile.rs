use std::fs::OpenOptions;
use std::io::prelude::*;
use std::fs;

pub fn delete_logfiles() {
    if let Err(e) = fs::remove_file("logfiles/missing_life_cycle.txt") {
    }

    if let Err(e) = fs::remove_file("logfiles/new_part_names.txt") {
    }
}

pub fn missing_life_cycle(sap_number: &str, mat_status: &str, name: &str) {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("logfiles/missing_life_cycle.txt")
        .unwrap();

    
    if let Err(e) = writeln!(file, "{}\t{}\t{}", sap_number, mat_status, name) {
        eprintln!("Couldn't write to file: {}", e);
    }
}