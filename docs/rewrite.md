# Complete ReWrite Guide

## Invoice class
    remains roughly the same
    does not contain any path information
    ensure that it is more functional (less side-effects)

## Setup class
    runs a setup function each time the program runs
        SETUP FUNCTION
            get the proper data directory
            does the mimanager section exist?
            does the config file exist?
            does the stored json file exist?
            does the backup folder exist?
    dir::data_local_dir

## Config class
    improve config file info (create less options)
    be able to change config values from the interface

## API interface
    creates all of the functions of the command line interface (including validation)
    works as a 

