use std::path::Path;
use std::fs;

fn get_platform_data_path() -> &'static Path {
    let config_dir = if cfg!(windows) {
        "%APPDATA%\\mimanager\\"
    } else {
        "/home/matt/.config/mimanager/"
    };
    Path::new(config_dir)
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
pub fn create_missing_directories_and_files() {}


// create backups of the billed json file
pub fn manage_backups() {
     
}
