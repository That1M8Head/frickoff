use dirs::{home_dir, config_dir};
use crate::config;
use crate::config::Config;
use crate::structs::Messages;
use std::path::PathBuf;
use std::collections::HashMap;

/// Checks that the number of command-line arguments is correct.
///
/// If not, prints an error message and returns `false`, indicating
/// that Frick Off should exit.
/// 
/// # Arguments
/// * `args` - The command-line arguments.
/// * `messages` - The `Messages` struct containing the error messages.
/// 
/// # Returns
/// * `bool` - `true` if the number of arguments is correct, `false` otherwise.
pub fn check_argument_count(args: &[String], messages: &Messages) -> bool {
    let only_options = args.len() == 2 && args[1].starts_with("--");
    if args.len() < 2 || only_options {
        if only_options && args[1] == "--no-jokes" {
            println!("{}", messages.serious.get("no_argument").unwrap());
        }
        else {
            println!("{}", messages.jokey.get("no_argument").unwrap());
        }
        println!("Usage: {} utility_name [--options]", args[0]);
        return false;
    }
    true
}

/// Uses a customisable prompt to confirm an action.
/// 
/// # Arguments
/// * `prompt` - The prompt to display.
/// * `confirmation` - The string you need to type to confirm.
/// * `confirmation_prompt` - The prompt to display when confirming.
/// * `messages` - The `Messages` struct containing the error messages.
/// * `options` - The command-line options.
/// 
/// # Returns
/// * `bool` - `true` if the user confirms the action, `false` otherwise.
pub fn confirm_with_prompt(prompt: &str, confirmation: &str, confirmation_prompt: &str, messages: &HashMap<&str, &str>, options: &[String]) -> bool {
    println!("{}", prompt);
    if options.contains(&"--no-confirm".to_string()) {
        return true;
    }
    println!("{}", confirmation_prompt);
    let mut to_quit = String::new();
    std::io::stdin().read_line(&mut to_quit).unwrap();
    if confirmation.len() == 1 {
        to_quit = to_quit.to_lowercase();
    }
    if to_quit.trim() != confirmation {
        println!("{}", messages.get("cancelled").unwrap());
        return true;
    }
    false
}

/// Creates the config file if it doesn't exist.
/// 
/// # Arguments
/// * `config_file_path` - The path to the config file.
/// 
/// # Returns
/// * `()` - Nothing.
pub fn create_config_if_needed(config_file_path: &PathBuf) {
    if !config::config_exists(config_file_path) {
        println!("No config exists - creating one.");
        config::create_config(config_file_path);
    }
}

/// Determines which confirmation message should be used depending on
/// the `paranoid` option in the config file or the command-line option.
/// 
/// # Arguments
/// * `options` - The command-line options.
/// * `configuration` - The `Config` struct containing the config file.
/// 
/// # Returns
/// * `(&'static str, &'static str)` - The confirmation message and prompt.
pub fn determine_confirmation(options: &[String], configuration: &Result<Config, toml::de::Error>) -> (&'static str, &'static str) {
    if options.contains(&"--paranoid".to_string()) || configuration.as_ref().unwrap().frickoff.paranoid {
        ("Yes, do as I say!", "To continue type in the phrase \"Yes, do as I say!\"")
    }
    else {
        ("y", "(Y/N)")
    }
}

/// Determines which message should be used depending on the `serious` config
/// option or `no-jokes` command-line option.
/// 
/// # Arguments
/// * `options` - The command-line options.
/// * `configuration` - The `Config` struct containing the config file.
/// * `messages` - The `Messages` struct containing the error messages.
/// 
/// # Returns
/// * `HashMap<&'static str, &'static str>` - The message map. Contains
/// what set of messages should be used.
pub fn determine_message_type(options: &[String], configuration: &Result<Config, toml::de::Error>, messages: &Messages) -> HashMap<&'static str, &'static str> {
    if options.contains(&"--no-jokes".to_string()) || configuration.as_ref().unwrap().frickoff.serious {
        messages.serious.clone()
    } else {
        messages.jokey.clone()
    }
}

/// Simply returns the currently implemented command-line options.
/// 
/// # Returns
/// * `Vec<&'static str>` - The command-line options.
pub fn get_valid_options() -> Vec<&'static str> {
    vec![
        "--and-init",   // Initialize a new Git repository automatically
        "--backup",     // Back up to HOME_PATH/frickoff-backups instead of deleting
        "--debug",      // Print debug info and quit
        "--help",       // Print a help message
        "--no-confirm", // Don't bother with the confirmation prompt
        "--no-jokes",   // "Serious" mode: uses the `serious` messages
        "--options",    // Print "That is VERY funny" and quit
        "--paranoid",   // Change "Enter Y/N" to "Enter 'Yes, do as I say!"
        "--verbose",    // Verbose output
    ]
}

/// Assigns the command-line arguments to variables and returns them as a tuple.
/// 
/// # Arguments
/// * `args` - The command-line arguments.
/// 
/// # Returns
/// * `(String, Vec<String>)` - The utility name and the command-line options.
pub fn handle_command_line_args(args: &[String]) -> (String, Vec<String>) {
    let utility_name = args[1].clone();
    let options = args[2..].to_vec();
    (utility_name, options)
}

/// Handles the command-line options.
/// 
/// # Arguments
/// * `utility_name` - The name of the utility.
/// * `home_path` - The path to the user's home directory.
/// * `config_path` - The path to the user's config directory.
/// * `messages` - The `Messages` struct containing the error messages.
/// * `configuration` - The `Config` struct containing the config file.
/// * `valid_options` - The valid command-line options.
/// * `options` - The command-line options.
///
/// (God, that's a lot of parameters)
/// 
/// # Returns
/// * `()` - Nothing.
pub fn handle_options(utility_name: &str, home_path: &PathBuf, config_path: &PathBuf, messages: &HashMap<&str, &str>, configuration: Result<Config, toml::de::Error>, valid_options: &[&str], options: &[String], ) {
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
                print_debug_info(&utility_name, &options, &home_path, &config_path, &messages, config);
                std::process::exit(0);
            }
            Err(err) => {
                eprintln!("Error reading config file: {}", err);
                std::process::exit(1);
            }
        }
    }
}

/// Makes sure that the config file exists and then reads and returns it.
/// 
/// # Arguments
/// * `config_file_path` - The path to the config file.
/// 
/// # Returns
/// * `Result<Config, toml::de::Error>` - The config file.
pub fn initialise_config(config_file_path: &PathBuf) -> Result<Config, toml::de::Error> {
    create_config_if_needed(&config_file_path);
    config::read_config(&config_file_path)
}


/// Prints debug information about the program.
/// 
/// # Arguments
/// Yes.
/// 
/// ## Okay, but seriously...
/// * `utility_name` - The name of the utility.
/// * `options` - The command-line options.
/// * `home_path` - The path to the user's home directory.
/// * `config_path` - The path to the user's config directory.
/// * `messages` - The `Messages` struct containing the error messages.
/// * `config` - The `Config` struct containing the config file.
/// 
/// # Returns
/// `()` - Nothing.
pub fn print_debug_info(utility_name: &str, options: &[String], home_path: &PathBuf, config_path: &PathBuf, messages: &HashMap<&str, &str>, config: Config) {
    println!("Chosen utility's name: {}", utility_name);
    println!("Command line options: {:#?}", options);
    println!("User home path: {}", home_path.display());
    println!("Configuration path: {}", config_path.display());
    println!("Messages: {:#?}", messages);
    println!("Config: {:#?}", config);
}

/// Sets the home and configuration directory paths.
/// 
/// # Returns
/// * `(PathBuf, PathBuf)` - The home and configuration directory paths.
pub fn set_paths() -> (PathBuf, PathBuf) {
    let home_path = home_dir().expect("Failed to get home directory");
    let config_path = config_dir().expect("Failed to get config directory");
    (home_path, config_path)
}
