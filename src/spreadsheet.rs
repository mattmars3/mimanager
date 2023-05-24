use spreadsheet_ods::{WorkBook, Sheet, Value, write_ods};
use spreadsheet_ods::format;
use spreadsheet_ods::formula;
use spreadsheet_ods::{cm, mm};
use spreadsheet_ods::style::{CellStyle};
use spreadsheet_ods::style::units::{TextRelief, Border, Length};

use icu_locid::Locale;

use crate::{invoice::{InvoiceHistory, WorkDay}, config::get_config_val};
use std::path::{Path, PathBuf};

use chrono::{Utc, TimeZone};



pub fn invoice_manager_to_ods(inv_hist: InvoiceHistory) {
    let spreadsheet_output = get_config_val("spreadsheet_output");
    let output_path = Path::new(&spreadsheet_output);
    let today_string = format!("{}{}", Utc::now().format("%d%b%Y-%H:%M").to_string(), ".ods");

    let mut file_path = PathBuf::new();
    file_path.push(output_path);
    file_path.push(Path::new(&today_string));

    if !output_path.exists() {
        match std::fs::create_dir(output_path) {
            Ok(_) => (),
            Err(e) => println!("{}", e),
        }
        
    }

    let mut spreadsheet: WorkBook = if output_path.exists() {
        let s = match spreadsheet_ods::read_ods(&file_path) {
            Ok(wb) => wb,
            Err(_) => {
                let loc: Locale = "en-US".parse().expect("Parsing key failed");
                WorkBook::new(loc)
            }
        };
        s
    };

    if spreadsheet.num_sheets() == 0 {
        let mut sheet = Sheet::new("simple");
        sheet.set_value(0, 0, true);
        spreadsheet.push_sheet(sheet);
    }

    println!("{}", file_path.to_string_lossy());
    write_ods(&mut spreadsheet, file_path);
}
