use inquire::{Text, DateSelect, Confirm};
use chrono::{NaiveDateTime, Utc, NaiveDate, NaiveTime, Duration};

use tabled::{Table, settings::Style};

use crate::{invoice::{WorkDay, InvoiceHistory}, config::get_config_val};

// print help message
fn manage_command_line_arguments() {
    let args: Vec<String> = std::env::args().collect();
    let help_message = "
Utilize multiple commands like the following:
    add => add a workday
    list => list recent workdays
    delete => delete a specific workday
    clear => clear all workdays
    ";
    for arg in args {
        if arg.to_lowercase().contains("help") {
            println!("{}", help_message);
        }
    }
}

pub fn run() {
    manage_command_line_arguments();
    let action: String = Text::new("What would you like to do?").prompt().unwrap();

    match action.as_str() {
        "add" => add_workday(),
        "list" => list_recent(),
        "clear" => clear_all(),
        "delete" => remove_by_id(),
        _ => println!("Invalid Option"),
    };
}

fn list_recent() {
    let invoice_history = InvoiceHistory::from_json();
    if invoice_history.is_empty() {
        println!("This invoice history has no entries!");
    } else {
        invoice_history.table_print_all();
    }
}

fn clear_all() {
    let mut invoice_history = InvoiceHistory::from_json();

    let message = format!("Are you sure you want to delete {} workdays?", invoice_history.num_invoices());
    let del_all = Confirm::new(message.as_str()).prompt().unwrap(); 

    if del_all {
        invoice_history.clear_all_workdays();
        invoice_history.write_to_json();
        println!("Cleared all Invoices and wrote to file!");
    } else {
        println!("Quit before deleting any workdays.");
    }

}

fn remove_by_id() {
    let mut invoice_history = InvoiceHistory::from_json();

    // if it's empty you can't remove anything
    if invoice_history.is_empty() {
        println!("Invoice history is already empty!");
    } else {
        list_recent();  
    }

    // enter id
    let remove_id_text = Text::new("What is ID of the invoice?").prompt().unwrap().parse::<i64>();
    match remove_id_text {
        Ok(id) => {
            match invoice_history.remove_workday(id) {
                Ok(_) => {
                    println!("Successfully removed workday!");
                    invoice_history.write_to_json();
                },
                Err(_) => println!("Failed to remove workday."),
            }
        },
        
        Err(e) => println!("Failed to parse id")
    };
}

fn add_workday() {
    let mut invoice_history = InvoiceHistory::from_json();
    let work_day = get_workday_from_user();

    match work_day {
        Ok(wd) => {
            wd.table_print();

            let write_workday: bool = Confirm::new("Is this information correct?").prompt().unwrap();

            if write_workday {
                invoice_history.add_workday(wd);
                invoice_history.write_to_json();
                println!("Successfully wrote to file!");
            }

        },
        Err(_) => {

        }
    }

}

// this function is nasty dude
fn get_workday_from_user() -> Result<WorkDay, ()> {
    let today: NaiveDateTime = Utc::now().naive_utc();
    // today's date for date recommendation
    let today_date: NaiveDate = today.date();

    // twelve hours to be added
    let twelve_hours = Duration::hours(12);

    // get the date you want to enter
    let date = DateSelect::new("Enter the date you want to log.")
        .with_default(today_date)
        .prompt().unwrap();

    let mut am_or_pms: Vec<&str> = vec![];

    let start_time_string = Text::new("Enter start time.").prompt().unwrap().to_uppercase();
    am_or_pms.push(if Confirm::new("PM?").prompt().unwrap() {"PM"} else {"AM"});

    let end_time_string = Text::new("Enter end time.").prompt().unwrap().to_uppercase();
    am_or_pms.push(if Confirm::new("PM?").prompt().unwrap() {"PM"} else {"AM"});

    let times = vec![start_time_string, end_time_string];
    let mut naive_datetimes: Vec<NaiveDateTime> = vec![];
    for entered_time_ind in 0..times.len() {
        // create a string with the PM or AM attached so it can detect it
        let str_time = format!("{} {}", times.get(entered_time_ind).unwrap(), am_or_pms.get(entered_time_ind).unwrap());

        // parse the times
        let parsed_time = date.and_time(NaiveTime::parse_from_str(str_time.as_str(), "%I:%M %p").expect("Failed to parse time"));

        // add them to the vector
        naive_datetimes.push(parsed_time);
    }

    // check if end time is earlier than start time
    // if it is then ask if it's the next day
    // if not, warn user that they can't enter negative hours
    let next_day = Confirm::new("The next day?").prompt().unwrap();

    ////////////////////////////////////////
    // check if hours are negative /////////
    ////////////////////////////////////////

    let default_rate = get_config_val("default_hourly_rate");
    let rate_string = Text::new("What is your hourly rate?").with_default(&default_rate).prompt().unwrap();
    let rate = rate_string.parse::<i32>().expect("Unable to parse hourly rate");
    
    let accomplishments = Text::new("What did you accomplish today? (Optional)")
        .prompt()
        .unwrap();


    // add twelve hours if PM
    for ind in 0..am_or_pms.len() {
        /*
        println!("{}", naive_datetimes[ind]);
        if am_or_pms[ind] == "PM" {
            naive_datetimes[ind] += twelve_hours;
        }
        if naive_datetimes[ind].format("%H").to_string() == "12".to_string() {
            naive_datetimes[ind] -= twelve_hours;
        }
        */
        // add 24 hours if the end time is the next day
        if next_day && ind == 1 {
            naive_datetimes[ind] += twelve_hours + twelve_hours;
        }
    }

    let workday = WorkDay::new(naive_datetimes[0], naive_datetimes[1], WorkDay::generate_id(), rate, accomplishments);
    Ok(workday)
}

