mod config;
mod command_interface;
mod graphical_interface;
mod setup;
mod invoice;

fn main() {
    setup::create_missing_directories_and_files();
    command_interface::run();
}



