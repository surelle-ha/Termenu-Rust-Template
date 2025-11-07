use crate::modules::termenu::Termenu;

pub fn register() -> Termenu {
    let command: Termenu = Termenu::new_command(
        "helloworld",
        "Print Hello, World!",
        |_options: &std::collections::HashMap<String, Option<String>>| {
            println!("Hello, World!");
            Ok(())
        },
    );

    command
}
