/// Frick Off (frickoff) - A utility that tells configuration files to frick
/// off and get deleted.
/// 
/// * Author: Arsalan "Velocity" Kazmi <sonicspeed848@gmail.com>
/// * License: GPLv3
/// * Repository: https://github.com/that1m8head/frickoff

use std::collections::HashMap;
mod functions;
mod config;
mod structs;
use functions::*;
use structs::*;

/// This function is called at the end of `main` to serve as a placeholder for
/// the actual functionality.
/// 
/// # Arguments
/// * `confirmation` - The string you need to type to confirm.
/// * `confirmation_prompt` - The prompt to display when confirming.
/// * `messages` - The `Messages` struct containing the error messages.
/// * `options` - The command-line options.
/// 
/// # Returns
/// * `()` - Nothing.
/// 
/// # Exits
/// * `0` - If the user confirms the action.
/// 
/// This is just a placeholder function, so it doesn't need to go in functions.rs.
fn placeholder_behaviour(confirmation: &str, confirmation_prompt: &str, messages: &HashMap<&str, &str>, options: &[String]) {
    println!("Apologies, but there's nothing useful beyond this point.");
    if confirm_with_prompt("Want to quit?", confirmation, confirmation_prompt, messages, options) {
        std::process::exit(0);
    }
}

/// I don't think I need to tell you what the main function is.
fn main() {
    let messages = Messages::new();
    let args: Vec<String> = std::env::args().collect();

    if !check_argument_count(&args, &messages) {
        std::process::exit(1);
    }

    let (home_path, config_path) = set_paths();
    let config_file_path = config_path.join("frickoff").join("config.toml");

    let configuration = initialise_config(&config_file_path);

    let (utility_name, options) = handle_command_line_args(&args);
    let (confirmation, confirmation_prompt) = determine_confirmation(&options, &configuration);

    let valid_options: Vec<&str> = get_valid_options();
    let messages = determine_message_type(&options, &configuration, &messages);

    handle_options(&utility_name, &home_path, &config_path, &messages, configuration, &valid_options, &options);

    placeholder_behaviour(confirmation, confirmation_prompt, &messages, &options);
}
