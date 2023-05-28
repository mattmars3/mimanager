use std::path::{Path, PathBuf};
use std::fs;
use dirs::data_local_dir;

fn get_platform_data_path() -> &'static Path {
    let mut data_dir: PathBuf = data_local_dir().unwrap();
    data_dir.push("mimanager");
    println!("{:?}", &data_dir);
    data_dir.as_path()
}

// checks if there is an assets folder. Creates it otherwise
pub fn get_assets_folder() -> &'static Path {
    let plat_data = get_platform_data_path();
    // create it if it doesn't exist
    if !plat_data.exists() {
        println!("The path {:?} does not exist.", plat_data.to_str());
        match fs::create_dir(plat_data) {
            Ok(_) => (),
            Err(e) => println!("ERROR {}", e),
        }
    }
    
    get_platform_data_path()
}

// finish this function
pub fn create_missing_directories_and_files() {

}


// create backups of the billed json file
pub fn manage_backups() {
     
}

pub fn setup() {

}
