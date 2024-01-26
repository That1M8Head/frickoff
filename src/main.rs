use std::path::PathBuf;
use std::collections::HashMap;
use config::Config;
use dirs::{home_dir, config_dir};
mod config;

struct Messages {
    jokey: HashMap<&'static str, &'static str>,
    serious: HashMap<&'static str, &'static str>,
}

impl Messages {
    fn new() -> Self {
        let jokey_messages: HashMap<&str, &str> = [
            ("cancelled", "Okay, I won't touch it."),
            ("confirm_deletion", "Tell UTILITY's config to frick off? This will remove all data in UTILITY_CONFIG_PATH and remove the folder itself."),
            ("failure", "Could not tell UTILITY to frick off, because ERROR."),
            ("literal_options", "...That is VERY funny."),
            ("no_argument", "What would you like to frick off?"),
            ("no_config", "There is no data in UTILITY's config, so it cannot frick off."),
            ("no_such_utility", "I don't know about a utility named UTILITY. You should add it in FRICKOFF_CONFIG_PATH."),
            ("success", "Fricking-off complete. Want to make a new Git repository in UTILITY_CONFIG_PATH?"),
        ].iter().cloned().collect();

        let serious_messages: HashMap<&str, &str> = [
            ("cancelled", "Operation cancelled."),
            ("confirm_deletion", "Remove all data in UTILITY_CONFIG_PATH? This action is not reversible."),
            ("failure", "Deletion unsuccessful. Reason: ERROR."),
            ("literal_options", "User attempted to literally use --options. They must think they're very clever."),
            ("no_argument", "No utility specified. Cannot proceed."),
            ("no_config", "UTILITY_CONFIG_PATH does not exist or the directory is empty."),
            ("no_such_utility", "No such utility named UTILITY. Consider adding its path in FRICKOFF_CONFIG_PATH."),
            ("success", "Deletion succeeded. Initialise a new Git repository at UTILITY_CONFIG_PATH?"),
        ].iter().cloned().collect();

        Messages {
            jokey: jokey_messages,
            serious: serious_messages
        }
    }
}

fn print_debug_info(utility_name: &str, options: &[String], home_path: &PathBuf, config_path: &PathBuf, messages: &HashMap<&str, &str>, config: Config) {
    println!("Chosen utility's name: {}", utility_name);
    println!("Command line options: {:#?}", options);
    println!("User home path: {}", home_path.display());
    println!("Configuration path: {}", config_path.display());
    println!("Messages: {:#?}", messages);
    println!("Config: {:#?}", config);
}

fn main() {
    let messages = Messages::new();

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

    if !config::config_exists(&config_file_path) {
        println!("No config exists - creating one.");
        config::create_config(&config_file_path);
    }

    let configuration = config::read_config(&config_file_path);

    let args: Vec<String> = std::env::args().collect();
    let only_options = args.len() == 2 && args[1].starts_with("--");
    if args.len() < 2 || only_options {
        if only_options && args[1] == "--no-jokes" {
            println!("{}", messages.serious.get("no_argument").unwrap());
        }
        else {
            println!("{}", messages.jokey.get("no_argument").unwrap());
        }
        println!("Usage: {} utility_name [--options]", args[0]);
        std::process::exit(1);
    }

    let utility_name = &args[1];
    let options = &args[2..];

    let messages = if options.contains(&"--no-jokes".to_string()) || configuration.as_ref().unwrap().frickoff.serious {
        &messages.serious
    } else {
        &messages.jokey
    };

    let (confirmation, confirmation_prompt) = if options.contains(&"--paranoid".to_string()) || configuration.as_ref().unwrap().frickoff.paranoid {
        ("Yes, do as I say!", "To continue type in the phrase \"Yes, do as I say!\"")
    }
    else {
        ("Y", "(Y/N)")
    };

    if options.contains(&"--options".to_string()) {
        println!("{}", messages.get("literal_options").unwrap());
        std::process::exit(0);
    }

    if options.len() > 1 && !valid_options.contains(&options[0].as_str()) {
        println!("Invalid option: {}", options[0]);
        std::process::exit(1);
    }


    if options.contains(&"--debug".to_string()) {
        match configuration {
            Ok(config) => {
                print_debug_info(utility_name, options, &home_path, &config_path, messages, config);
                std::process::exit(0);
            }
            Err(err) => {
                eprintln!("Error reading config file: {}", err);
                std::process::exit(1);
            }
        }
    }

    println!("Apologies, but there's nothing useful beyond this point.");
    println!("Want to quit?");

    println!("{}", confirmation_prompt);
    let mut to_quit = String::new();
    std::io::stdin().read_line(&mut to_quit).unwrap();
    if confirmation.len() == 1 {
        to_quit = to_quit.to_lowercase();
    }
    if to_quit.trim() != confirmation {
        println!("{}", messages.get("cancelled").unwrap());
        std::process::exit(0);
    }
}
