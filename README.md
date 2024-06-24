# Task Reminder CLI Application

This is a simple CLI application that allows users to input tasks, save them in a JSON file, and receive email reminders based on the frequency they choose.

## Features

**Input tasks:** Users can input their tasks via the command line interface in Rust.
**Save tasks:** Tasks are saved in a 'data.json' file for persistent storage.
**Email reminders:** A Python script reads the 'data.json' file, processes reminders, and sends emails based on user preferences.

## Components

### Rust CLI

The Rust portion of the application handles user input and saves the data into 'data.json'.

### Python Email Sender

The Python script reads the 'data.json' file, schedules reminders, and sends emails.

## Usage

**Running the project:** Execute ./run.sh in your terminal to start the application.

## Requirements

   - **Rust:**
        - pyo3 = "0.21.1"
            For integrating with Python in Rust
        
        - colored = "2.1.0" 
            For adding color to terminal output
        
        - regex = "1.5" 
            For regular expression support
        
        - serde = { version = "1.0", features = ["derive"] } 
            For serialization/deserialization
        
        - serde_json = "1.0" 
            For JSON support with Serde
        
        - clearscreen = "2.0.1" 
            For clearing the terminal screen

   - **Python 3.x:**
        - maturin = "0.12.0" 
            Maturin for building Rust crates as Python packages
          
## Note
   **The project is currently under development.**
   
### Peace.
