use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Profile {
    pub variables: HashMap<String, String>,
}

impl Profile {
    pub fn new() -> Self {
        let mut variables = HashMap::new();
        // Global defaults as requested by user
        variables.insert("fontsize".to_string(), "10pt".to_string());
        variables.insert("lang".to_string(), "en".to_string());
        variables.insert("paper".to_string(), "a4".to_string());
        // region: none - in Typst template this typically means no region variable, 
        // or we can set it explicitly if needed.
        
        Self { variables }
    }

    pub fn apply_preset(&mut self, preset: &str) {
        match preset.to_lowercase().as_str() {
            "academic" => {
                self.variables.insert("cols".to_string(), "1".to_string());
                self.variables.insert("fontsize".to_string(), "11pt".to_string());
                self.variables.insert("font".to_string(), "serif".to_string());
                self.variables.insert("sectionnumbering".to_string(), "1.1.1".to_string());
            }
            "technical" => {
                self.variables.insert("cols".to_string(), "2".to_string());
                self.variables.insert("fontsize".to_string(), "9pt".to_string());
                self.variables.insert("font".to_string(), "sans-serif".to_string());
                self.variables.insert("margin.x".to_string(), "0.75in".to_string());
                self.variables.insert("margin.y".to_string(), "0.75in".to_string());
            }
            "manuscript" => {
                self.variables.insert("fontsize".to_string(), "12pt".to_string());
                self.variables.insert("linestretch".to_string(), "2".to_string());
                self.variables.insert("margin.x".to_string(), "1.5in".to_string());
                self.variables.insert("margin.y".to_string(), "1.5in".to_string());
            }
            "report" => {
                // Default report settings or just use global defaults
                self.variables.insert("sectionnumbering".to_string(), "1.1".to_string());
            }
            "memo" => {
                self.variables.insert("fontsize".to_string(), "11pt".to_string());
                self.variables.insert("paper".to_string(), "us-letter".to_string());
            }
            "letter" => {
                self.variables.insert("fontsize".to_string(), "12pt".to_string());
                self.variables.insert("margin.x".to_string(), "1in".to_string());
                self.variables.insert("margin.y".to_string(), "1in".to_string());
            }
            _ => {}
        }
    }

    pub fn override_variable(&mut self, key: &str, value: &str) {
        self.variables.insert(key.to_string(), value.to_string());
    }
}
