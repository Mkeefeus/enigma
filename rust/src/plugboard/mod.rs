use std::collections::HashMap;

#[derive(Debug)]
pub struct Plugboard {
    pub mappings: HashMap<char, char>

}

impl Plugboard {
    pub fn new() -> Plugboard {
        Plugboard { mappings: HashMap::new() }
    }
}
