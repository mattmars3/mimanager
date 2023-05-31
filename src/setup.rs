use std::path::{Path, PathBuf};
use std::fs;
use dirs::data_local_dir;
use log::{info, error};

// checks if there is an assets folder. Creates it otherwise
pub fn get_assets_folder() -> PathBuf {
    let mut data_dir: PathBuf = data_local_dir().unwrap();
    data_dir.push("mimanager");
    // create it if it doesn't exist
    if !data_dir.exists() {
        match fs::create_dir(data_dir.clone()) {
            Ok(_) => (),
            Err(e) => error!("{}", e),
        }
    }

    // set storage file path in config file
    data_dir 
}


// finish this function
pub fn create_missing_directories_and_files() {
    let assets_folder = get_assets_folder();
    // create assets directory
    match assets_folder.exists() {
        true => (),
        false => {
            match fs::create_dir(assets_folder) {
                Ok(_) => info!("Created assets folder"),
                Err(e) => error!("Unable to create assets folder. {}", e),
            }
        }
    }
}


// create backups of the billed json file
pub fn manage_backups() {
     
}

pub fn setup() {

}
