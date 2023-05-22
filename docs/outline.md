# Invoice Manager Project Outline
## Requirements
    Graphical Interface
        Enter Start and End Hours
        Optionally Add Accomplishments
    Outputs Formatted Spreadsheet

## Technical Info & Implementation
### Libraries
    spreadsheet-ods
    Dioxus (GUI)

    Detect whether on linux or windows machine
    
### Information 
    Cross-platform interface/app because I use (linux) and others (windows)
    Use ODS File format instead of XLS because it is open and more available
    

## Program Cycle
    Loads data from config file
    Decide what the user wants to do
        Write information to spreadsheet
        Enter in day's information
            Take in Information
            Write the information to the json file (path defined in config)
