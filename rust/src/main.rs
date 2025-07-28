use dialoguer::Input;
use dialoguer::MultiSelect;
use std::collections::HashMap;
// use dialoguer::Select;

fn rotor_prompt(rotors: &[&str]) -> Vec<usize> {
    let rotor_selections: Vec<usize> = MultiSelect::new()
        .with_prompt("Select 3 rotors to use")
        .items(&rotors)
        .interact()
        .unwrap();

    if rotor_selections.len() != 3 {
        println!("Invalid number of rotors, select 3 rotors to use.");
        return rotor_prompt(rotors);
    }
    return rotor_selections;
}

fn rotor_setting_prompt(rotor_name: &&str) -> i8 {
    let setting_selection: i8 = Input::new()
        .with_prompt(format!("Select a setting for rotor {} (1-26)", *rotor_name))
        .validate_with(|input: &i8| -> Result<(), &str> {
            if *input < 1 || *input > 26 {
                Err("Invalid rotor setting. Please enter a number between 1 and 26 inclusive.")
            } else {
                Ok(())
            }
        })
        .interact_text()
        .unwrap();
    return setting_selection;
}

fn plugboard_prompt(plugboard: &mut HashMap<char, char>) {
    let cables_to_use: i8 = Input::new()
        .with_prompt("How many plugboard settings would you like? (0-10)")
        .validate_with(|input: &i8| -> Result<(), &str> {
            if *input < 0 || *input > 10 {
                Err("Invalid number of plugboard settings. Please enter a number between 0 and 10 inclusive.")
            } else {
                Ok(())
            }
        })
        .interact_text()
        .unwrap();
    if cables_to_use == 0 {
        println!("Continuing with no plugboard cables");
        return;
    }
    for _ in 0..cables_to_use {
        let start_point: char = Input::new()
            .with_prompt("Enter the character you would like to change (A-Z)")
            .validate_with(|input: &char| -> Result<(), &str> {
                if input.is_ascii_alphabetic() {
                    Ok(())
                } else {
                    Err("Invalid character. Please enter an English letter A-Z")
                }
            })
            .interact_text()
            .unwrap()
            .to_ascii_uppercase();
        let end_point: char = Input::new()
            .with_prompt("Enter the character you would like to map the first character to (A-Z)")
            .validate_with(|input: &char| -> Result<(), &str> {
                if *input != start_point && input.is_ascii_alphabetic() {
                    Ok(())
                } else if *input == start_point {
                    Err("Ending character cannot be the same as the starting character")
                } else {
                    Err("Invalid character. Please enter an English letter A-Z")
                }
            })
            .interact_text()
            .unwrap()
            .to_ascii_uppercase();
        plugboard.insert(start_point, end_point);
        plugboard.insert(end_point, start_point);
    }
}

fn main() {
    println!("Welcome to Enigma");
    // let versions: [&'static str; 1] = ["Enigma I"];
    // let enigma_selection: usize = Select::new()
    //     .with_prompt("Select which version of Enigma you would like to use. Use arrow keys to select and Enter to confirm")
    //     .items(&versions)
    //     .interact()
    //     .unwrap();

    // let enigma_version: &'static str = versions[enigma_selection];

    let rotors: [&'static str; 5] = ["I", "II", "III", "IV", "V"];

    let rotor_selections: Vec<usize> = rotor_prompt(&rotors);

    let mut rotor_settings: HashMap<usize, i8> = HashMap::new();

    for rotor in rotor_selections {
        rotor_settings.insert(rotor, rotor_setting_prompt(&rotors[rotor]));
    }

    println!("Rotor settings:",);
    for (rotor, setting) in rotor_settings {
        println!("Rotor {}: {}", rotors[rotor], setting)
    }

    let mut plugboard_settings: HashMap<char, char> = HashMap::new();

    plugboard_prompt(&mut plugboard_settings);

    println!("Plugboard settings:");
    for (start, end) in plugboard_settings {
        println!("{} -> {}", start, end)
    }
}
