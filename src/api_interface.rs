use inquire::{Text, DateSelect, Confirm};
use chrono::{NaiveDateTime, Utc, NaiveDate, NaiveTime, Duration};

use tabled::{Table, settings::Style};

use crate::{invoice::{WorkDay, InvoiceHistory}, config::get_config_val};
use crate::spreadsheet;


pub fn add_workday(unparsed_start_date: String, unparsed_end_date: String, unparsed_rate: String, accomplishments: String) -> Result<String, ()> {
    let invoice_history = InvoiceHistory::from_json();
    let new_workday = WorkDay::validated_new();
    let workday = if let Ok(wd) = new_workday {
        wd
    } else {
        return Err(());
    };

    let wt = new_workday.work_time();
    let time_worked = format!("{} hours and {} minutes", wt.0, wt.1);
    invoice_history.add_workday(new_workday);
    invoice_history.write_to_json();
    Ok(time_worked)
}

pub fn remove_workday(id: String) -> Result<(), String> {
    let invoice_history = InvoiceHistory::from_json();
    let wd_id: i64 = match id.parse() {
        Ok(id) => id,
        Err(_) => return Err("Failed to parse id".to_string()),
    };
    match invoice_history.remove_workday(wd_id) {
        Ok(_) => {
            invoice_history.write_to_json();
            return Ok(())
        },
        Err(_) => return Err("Failed to remove workday from InvoiceManager"),
    }
}
pub fn clear_workdays() {
    let invoice_history = InvoiceHistory::from_json();
    invoice_history.clear_all_workdays();
    invoice_history.write_to_json();
}

pub fn output_to_invoice() {}
pub fn earnings_total() -> f64 {

}


