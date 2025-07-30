mod enigma_i;
use crate::plugboard::Plugboard;
use crate::rotor::Rotor;

pub struct Enigma {
    rotors: Vec<Rotor>,
    plugboard: Plugboard,
    edition: EnigmaType, // Use enum instead of String
}

pub enum EnigmaType {
    EnigmaI,
    // EnigmaM3,
    // EnigmaM4,
}

pub trait EnigmaMachine {
    fn encrypt(&mut self, plaintext: &str) -> String;
    fn decrypt(&mut self, ciphertext: &str) -> String;
    fn generate_rotors(&mut self) -> Option<()>;
    fn generate_plugboard(&mut self) -> ();
}

impl EnigmaMachine for Enigma {
    fn encrypt(&mut self, plaintext: &str) -> String {
        match self.edition {
            EnigmaType::EnigmaI => self.encrypt_enigma_i(plaintext),
        }
    }

    fn decrypt(&mut self, ciphertext: &str) -> String {
        match self.edition {
            EnigmaType::EnigmaI => self.decrypt_enigma_i(ciphertext),
        }
    }

    fn generate_rotors(&mut self) -> Option<()> {
        match self.edition {
            EnigmaType::EnigmaI => self.generate_rotors_enigma_i(),
        }
    }
    fn generate_plugboard(&mut self) -> () {
        match self.edition {
            EnigmaType::EnigmaI => self.generate_plugboard_enigma_i(),
        }
    }
}

impl Enigma {
    pub fn new(edition: EnigmaType) -> Enigma {
        Enigma {
            rotors: Vec::new(),
            plugboard: Plugboard::new(),
            edition,
        }
    }

    pub fn configure(&mut self) -> Result<(), String> {
        self.generate_plugboard();
        self.generate_rotors().ok_or("Failed to generate rotors")?;
        Ok(())
    }
}
