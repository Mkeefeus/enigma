use crate::rotor::{Rotor, RotorData};
use dialoguer::{Input, Select};

use super::Enigma;

impl Enigma {
    pub(super) fn encrypt_enigma_i(&mut self, plaintext: &str) -> String {
        // Enigma I specific encryption logic here
        todo!("Implement Enigma I encryption")
    }

    pub(super) fn decrypt_enigma_i(&mut self, ciphertext: &str) -> String {
        // Enigma I specific decryption logic here
        todo!("Implement Enigma I decryption")
    }

    pub(super) fn generate_rotors_enigma_i(&mut self) -> Option<()> {
        let mut rotor_types: Vec<&str> = vec!["I", "II", "III", "IV", "V"];
        let itr_text_conv: [&'static str; 3] = ["1st", "2nd", "3rd"];
        for i in 0..3 {
            let rotor_selection: usize = Select::new()
                .with_prompt(format!("Select {} rotor", &itr_text_conv[i]))
                .items(&rotor_types)
                .interact()
                .unwrap();
            let setting_selection: i8 = Input::new()
            .with_prompt(format!("Select a setting for new rotor (1-26)"))
            .validate_with(|input: &i8| -> Result<(), &str> {
                if *input < 1 || *input > 26 {
                    Err("Invalid rotor setting. Please enter a number between 1 and 26 inclusive.")
                } else {
                    Ok(())
                }
            })
            .interact_text()
            .unwrap();
            let new_rotor: Option<Rotor> = Rotor::new(RotorData {
                version: rotor_types[rotor_selection].to_string(),
                slot: i as i8,
                position: setting_selection,
            });

            if new_rotor.is_none() {
                eprintln!("Failed to create rotor");
                return None;
            }

            self.rotors.insert(self.rotors.len(), new_rotor.unwrap());
            rotor_types.remove(rotor_selection);
        }
        println!("Rotors: {:#?}", self.rotors);
        Some(())
    }

    pub(super) fn generate_plugboard_enigma_i(&mut self) -> () {
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
                .with_prompt(
                    "Enter the character you would like to map the first character to (A-Z)",
                )
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
            self.plugboard.mappings.insert(start_point, end_point);
            self.plugboard.mappings.insert(end_point, start_point);
        }
        println!("Plugboard: {:#?}", self.plugboard.mappings)
    }

    // Other Enigma I specific methods
}
