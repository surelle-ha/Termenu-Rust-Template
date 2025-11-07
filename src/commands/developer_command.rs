use crate::modules::exceptions::ServError;
/**
 * =========================================
 * Note: This is an internal command intended
 * for development use only. You may remove it
 * depending on your specific use case or how
 * you plan to use this tool.
 * =========================================
 */
use crate::modules::termenu::Termenu;
use colored::Colorize;
use serde_json::json;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub fn register() -> Termenu {
    let mut command: Termenu = Termenu::new_command(
        "developer",
        "Add new command for development purposes. Will only work in debug mode.",
        |options: &HashMap<String, Option<String>>| {
            if cfg!(debug_assertions) {
                let verbose_mode: bool = options.contains_key("--verbose");

                let command_name: Option<String> = options
                    .get("--add")
                    .and_then(|v: &Option<String>| v.as_ref())
                    .map(|v: &String| v.trim().to_string());

                let command_name = match command_name {
                    Some(name) if !name.is_empty() => name,
                    _ => {
                        return Err(ServError::input_missing_error(Some(json!({
                            "issue": "Action is required. Use `--add=<command_name>` to continue."
                        }))));
                    }
                };

                if verbose_mode {
                    println!("{} Verbose mode enabled.", "✔".green());
                    println!("{} Adding command: {}", "ℹ".cyan(), command_name);
                }

                let file_name: String =
                    format!("src/commands/{}_command.rs", command_name.to_lowercase());
                let path: &Path = Path::new(&file_name);

                if path.exists() {
                    return Err(ServError::input_unknown_error(Some(json!({
                        "issue": format!("Command file already exists: {}", file_name)
                    }))));
                }

                fs::write(&path, generate_command_template(&command_name)).map_err(|e| {
                    ServError::connection_unknown_error(Some(json!({
                        "issue": format!("Failed to create file: {}", e)
                    })))
                })?;

                if verbose_mode {
                    println!(
                        "{} Command file created successfully at: {}. \
                         Register the new command manually on the entrypoint.",
                        "✔".green(),
                        file_name
                    );
                }

                Ok(())
            } else {
                Err(ServError::invalid_command_error(Some(json!({
                    "issue": "This command is only available in debug mode."
                }))))
            }
        },
    );

    command.add_option(
        "--verbose",
        "--verbose | Display detailed usage information.",
    );
    command.add_option(
        "--add",
        "--add=<command_name> | Specify the name of the command to add.",
    );

    command
}

fn generate_command_template(name: &str) -> String {
    let _struct_name: String = format!("{}Command", capitalize_first_letter(&name.to_lowercase()));
    format!(
        "use crate::modules::termenu::Termenu;\n\n\
        pub fn register() -> Termenu {{\n    \
            let command: Termenu = Termenu::new_command(\n        \
                \"{}\",\n        \
                \"Describe what this command does.\",\n        \
                |_options| {{\n            \
                    println!(\"Command '{}' executed.\");\n            \
                    Ok(())\n        \
                }},\n    );\n\n    \
            command\n\
        }}\n",
        name.to_lowercase(),
        name
    )
}

fn capitalize_first_letter(s: &str) -> String {
    let mut c: std::str::Chars<'_> = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
