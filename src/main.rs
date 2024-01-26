use dirs::{home_dir, config_dir};
mod functions;
mod config;
mod structs;
use functions::*;
use structs::*;

fn main() {
    let messages = Messages::new();
    let args: Vec<String> = std::env::args().collect();

    check_argument_count(&args, &messages);

    let utility_name = &args[1];
    let options = &args[2..];

    let valid_options: Vec<&str> = vec![
        "--and-init",   // Initialize a new Git repository automatically
        "--backup",     // Back up to HOME_PATH/frickoff-backups instead of deleting
        "--debug",      // Print debug info and quit
        "--help",       // Print a help message
        "--no-confirm", // Don't bother with the confirmation prompt
        "--no-jokes",   // "Serious" mode: uses the `serious` messages
        "--options",    // Print "That is VERY funny" and quit
        "--paranoid",   // Change "Enter Y/N" to "Enter 'Yes, do as I say!"
        "--verbose",    // Verbose output
    ];

    let home_path = home_dir().expect("Failed to get home directory");
    let config_path = config_dir().expect("Failed to get config directory");
    let config_file_path = config_path.join("frickoff").join("config.toml");

    create_config_if_needed(&config_file_path);
    let configuration = config::read_config(&config_file_path);

    let messages = determine_message_type(options, &configuration, &messages);

    let (confirmation, confirmation_prompt) = determine_confirmation(options, &configuration);

    if options.contains(&"--options".to_string()) {
        println!("{}", messages.get("literal_options").unwrap());
        std::process::exit(1);
    }

    if options.len() > 1 && !valid_options.contains(&options[0].as_str()) {
        println!("Invalid option: {}", options[0]);
        std::process::exit(1);
    }

    if options.contains(&"--debug".to_string()) {
        match configuration {
            Ok(config) => {
                print_debug_info(utility_name, options, &home_path, &config_path, &messages, config);
                std::process::exit(0);
            }
            Err(err) => {
                eprintln!("Error reading config file: {}", err);
                std::process::exit(1);
            }
        }
    }

    placeholder_behaviour(confirmation, confirmation_prompt, &messages);
}
