use chrono::{NaiveDate};

pub fn print_data(message: &str, data_str: &str) {
    let data_original = NaiveDate::parse_from_str(data_str, "%Y%m%d").unwrap();
    println!("{}: {}",message,data_original.format("%d/%m/%Y").to_string());
}

