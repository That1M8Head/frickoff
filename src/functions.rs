use crate::config;
use crate::config::Config;
use crate::structs::Messages;
use std::path::PathBuf;
use std::collections::HashMap;

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

pub fn create_config_if_needed(config_file_path: &PathBuf) {
    if !config::config_exists(config_file_path) {
        println!("No config exists - creating one.");
        config::create_config(config_file_path);
    }
}

pub fn determine_confirmation(options: &[String], configuration: &Result<Config, toml::de::Error>) -> (&'static str, &'static str) {
    if options.contains(&"--paranoid".to_string()) || configuration.as_ref().unwrap().frickoff.paranoid {
        ("Yes, do as I say!", "To continue type in the phrase \"Yes, do as I say!\"")
    }
    else {
        ("y", "(Y/N)")
    }
}

pub fn determine_message_type(options: &[String], configuration: &Result<Config, toml::de::Error>, messages: &Messages) -> HashMap<&'static str, &'static str> {
    if options.contains(&"--no-jokes".to_string()) || configuration.as_ref().unwrap().frickoff.serious {
        messages.serious.clone()
    } else {
        messages.jokey.clone()
    }
}

pub fn print_debug_info(utility_name: &str, options: &[String], home_path: &PathBuf, config_path: &PathBuf, messages: &HashMap<&str, &str>, config: Config) {
    println!("Chosen utility's name: {}", utility_name);
    println!("Command line options: {:#?}", options);
    println!("User home path: {}", home_path.display());
    println!("Configuration path: {}", config_path.display());
    println!("Messages: {:#?}", messages);
    println!("Config: {:#?}", config);
}
