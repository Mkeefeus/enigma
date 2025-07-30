mod plugboard;
mod rotor;
mod enigma;

use dialoguer::Select;
use enigma::{Enigma, EnigmaType};

fn main() {
    println!("Welcome to Enigma");
    
    // Let user select Enigma version
    let versions = ["Enigma I"]; // Add more as they are implemented
    let enigma_selection = Select::new()
        .with_prompt("Select which version of Enigma you would like to use")
        .items(&versions)
        .interact()
        .unwrap();
    
    let edition = match enigma_selection {
        0 => EnigmaType::EnigmaI,
        // 1 => EnigmaType::EnigmaM3,
        // 2 => EnigmaType::EnigmaM4,
        _ => {
            eprintln!("Invalid selection");
            return;
        }
    };
    
    let mut enigma_machine = Enigma::new(edition);
    
    println!("Enigma machine configured successfully!");

    let config_result = enigma_machine.configure();
}