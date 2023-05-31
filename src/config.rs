/* 
* Config values
*   Assets folder
*   Default Hourly Rate
*   run before?
* */

use serde_json::{Value, from_str};

use std::fs::{read_to_string, write};
use std::io::Error;

use crate::setup::get_assets_folder;

use std::path::PathBuf;

pub fn get_config_val(conf_val: &str) -> String {
    let config = read_config_file();

    let config_value: Value = from_str(&config).expect("Unparsable JSON. Error in the configuration file"); 
    let value: String = config_value[conf_val].to_string();
    let final_val = &value[1..value.len()-1];
    final_val.to_string()
}

fn read_config_file() -> String {
    let mut config_path_buf = PathBuf::from(get_assets_folder());
    config_path_buf.push("config.json");

    let config = match read_from_file(config_path_buf.clone()) {
        Ok(data) => data,
        Err(e) => {
            // if it doesn't exist add a blank dictionary to the json file
            match write(config_path_buf, "{}") {
                Ok(_) => (),
                Err(e) => println!("Failed to write blank dictionary to config path. {}", e),
            }
            "{
                \"default_hourly_rate\": \"25\",
                \"spreadsheet_output\": \"/home/matt/.config/mimanager/spreadsheets/\"
            }".to_string()
        }
    };
    config
}

fn read_from_file(path: PathBuf) -> Result<String, Error> {
    let data = read_to_string(path);
    data
}
