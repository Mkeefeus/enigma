use std::collections::HashMap;

#[derive(Debug)]
pub struct Rotor {
    pub wiring: String,
    pub version: String,
    pub slot: i8,
    pub position: i8
}

pub struct RotorData {
    pub version: String,
    pub slot: i8,
    pub position: i8
}
// Example with error handling (using Option)
impl Rotor {
    pub fn new(rotor_data: RotorData) -> Option<Rotor> {
        let rotor_mappings: HashMap<String, String> = HashMap::from([
            ("ETW".to_string(), "ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string()),
            ("I".to_string(), "EKMFLGDQVZNTOWYHXUSPAIBRCJ".to_string()),
            ("II".to_string(), "AJDKSIRUXBLHWTMCQGZNPYFVOE".to_string()),
            ("III".to_string(), "BDFHJLCPRTXVZNYEIWGAKMUSQO".to_string()),
            ("IV".to_string(), "ESOVPZJAYQUIRHXLNFTGKDCMWB".to_string()),
            ("V".to_string(), "VZBRGITYUPSDNHLXAWMJQOFECK".to_string()),
        ]);

        if let Some(wiring_string) = rotor_mappings.get(&rotor_data.version) {
            Some(Rotor {
                wiring: wiring_string.clone(),
                version: rotor_data.version,
                slot: rotor_data.slot,
                position: rotor_data.position
            })
        } else {
            eprintln!("Error: Rotor version '{}' not found.", rotor_data.version);
            None // Or return a Result
        }
    }
}