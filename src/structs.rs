use std::collections::HashMap;

/// The `Messages` struct contains Frick Off's messages.
/// 
/// # Fields
/// * `jokey` - A hashmap containing the jokey messages.
/// * `serious` - A hashmap containing the serious messages.
pub struct Messages {
    pub jokey: HashMap<&'static str, &'static str>,
    pub serious: HashMap<&'static str, &'static str>,
}

/// The implementation of the `Messages` struct.
/// 
/// # Methods
/// * `new()` - Creates a new `Messages` struct.
impl Messages {
    pub fn new() -> Self {
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