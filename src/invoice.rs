use serde::{Serialize, Deserialize};
use chrono::naive::NaiveDateTime;

use serde_json::{to_string, from_str};

use crate::config::get_config_val;

use tabled::{Tabled, builder::Builder, settings::Style};

use rand::Rng;

#[derive(Debug, Serialize, Deserialize, Tabled)]
pub struct WorkDay {
    start_time: NaiveDateTime,
    end_time: NaiveDateTime,
    id: i64,
    rate: i32,
    accomplishments: String,
}

impl WorkDay {
    pub fn new(start_time: NaiveDateTime, end_time: NaiveDateTime, id: i64, rate: i32, accomplishments: String) -> WorkDay {
        WorkDay { start_time, end_time, id, rate, accomplishments } 
    }

    // returns hours, minutes, and total minutes worked
    pub fn work_time(&self) -> (i64, i64, i64) {
        // total time in hours 
        let work_duration = self.end_time - self.start_time;
        let hours = work_duration.num_hours();
        let minutes = work_duration.num_minutes() - (hours * 60);
        (hours, minutes, work_duration.num_minutes())
    }

    // returns the amount of money made for a day
    pub fn pay_for_day(&self) -> f64 {
        let rate_per_min: f64 = self.rate as f64 / 60f64;
        let pay_decimal = self.work_time().2 as f64 * rate_per_min;
        let rounded_pay = (pay_decimal * 100.0).round() / 100.0;
        rounded_pay
    }

    // generates 6 digit id
    pub fn generate_id() -> i64 {
        let mut rng = rand::thread_rng();
        let id: i64 = rng.gen_range(100000..999999);
        id
    }

    pub fn get_id(&self) -> i64 {
        self.id
    }

    // nicely prints the workday for debug purposes

    pub fn table_print(&self) {
        let mut builder = Builder::default();
        builder.set_header(["Date", "Start Time", "End Time", "Hourly Rate", "Hours Worked", "Earnings", "ID"]);
        let info_vec = self.table_information();
        builder.push_record(info_vec);

        let mut table = builder.build();
        table.with(Style::rounded());
        println!("{}", table);
    }

    pub fn table_information(&self) -> Vec<String> {
        let mut info_vec: Vec<String> = vec![];
        let work_time = format!("{} hours {} mins", self.work_time().0, self.work_time().1);
        info_vec.push(self.start_time.format("%m/%d/%Y").to_string());
        info_vec.push(self.start_time.format("%I:%M %p").to_string());
        info_vec.push(self.end_time.format("%I:%M %p").to_string());
        info_vec.push(self.rate.to_string());
        info_vec.push(work_time);

        let pay = self.pay_for_day();
        let pay_string = format!("${}", pay);
        info_vec.push(pay_string);

        info_vec.push(self.id.to_string());

        info_vec
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InvoiceHistory {
    workdays: Vec<WorkDay>,
}

impl InvoiceHistory {
    pub fn new(workdays: Vec<WorkDay>) -> InvoiceHistory {
        InvoiceHistory { workdays }
    }

    pub fn add_workday(&mut self, wd: WorkDay) {
        self.workdays.push(wd);
    }

    pub fn total_pay(&self) -> f64 {
        let mut total = 0f64;
        for workday in &self.workdays {
            total += workday.pay_for_day();
        }
        total
    }

    pub fn from_json() -> InvoiceHistory {
        let storage_file_path: String = get_config_val("storage_file_path");

        let file_content = match std::fs::read_to_string(&storage_file_path) {
            Ok(data) => data,
            Err(e) => {
                println!("{}", e);
                std::fs::write(&storage_file_path, "{}").expect("Failed to create new bill storage file");
                "[]".to_string()
            },
        };
        let workdays: Vec<WorkDay> = from_str(&file_content).expect("Failed to parse file content");
        let invoice_obj = InvoiceHistory::new(workdays);
        invoice_obj
    }

    pub fn write_to_json(&self) {
        let json_invoices: String = to_string(&self.workdays).expect("Failed to serialize json invoices");        

        let storage_file_path: String = get_config_val("storage_file_path");

        match std::fs::write(storage_file_path, json_invoices) {
            Ok(_) => (),
            Err(e) => println!("ERROR: {}", e),
        };
    }

    // print a table of weekly statistics like hours worked, money earned, etc
    pub fn weekly_stats() {

    }

    pub fn table_print_all(&self) {
        let mut builder = Builder::default();
        builder.set_header(["Date", "Start Time", "End Time", "Hourly Rate", "Hours Worked", "Earnings", "ID"]);

        for workday in &self.workdays {
            let info_vec = workday.table_information();
            builder.push_record(info_vec);
        }

        let mut table = builder.build();
        table.with(Style::rounded());
        println!("{}", table);
    }

    pub fn remove_workday(&mut self, id: i64) -> Result<(), ()> {
        for ind in 0..self.workdays.len() {
            if self.workdays.get(ind).unwrap().get_id() == id {
                self.workdays.remove(ind);
                return Ok(());
            }
        }
        Err(())

    }

    pub fn clear_all_workdays(&mut self) {
        self.workdays = Vec::new();
    }

    pub fn num_invoices(&self) -> i32 {
        self.workdays.len() as i32 
    }

    pub fn is_empty(&self) -> bool {
        if self.workdays.len() == 0 {
            return true;
        }
        false
    }
}

///// TESTS TESTS TESTS TESTS 

#[test]
fn test_workday() {
    let start_time = NaiveDate::from_ymd_opt(2022, 11, 3).unwrap().and_hms_opt(9, 0, 0).unwrap();
    let end_time = NaiveDate::from_ymd_opt(2022, 11, 3).unwrap().and_hms_opt(12, 0, 0).unwrap();
    let sample_workday = WorkDay::new(start_time, end_time, 1i64, 25, "Test".to_string());
    
    assert_eq!(sample_workday.work_time().2, (3 * 60) as i64);
}

