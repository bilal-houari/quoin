use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Profile {
    pub variables: HashMap<String, String>,
}

impl Profile {
    pub fn new() -> Self {
        let mut variables = HashMap::new();
        // Global defaults
        variables.insert("fontsize".to_string(), "10pt".to_string());
        variables.insert("lang".to_string(), "en".to_string());
        variables.insert("papersize".to_string(), "a4".to_string());

        Self { variables }
    }

    pub fn apply_preset(&mut self, preset: &str) {
        match preset.to_lowercase().as_str() {
            "ultra-dense" => {
                self.variables.insert("fontsize".to_string(), "8pt".to_string());
                self.variables.insert("margin.x".to_string(), "2cm".to_string());
                self.variables.insert("margin.y".to_string(), "2cm".to_string());
            }
            "ultra-dense-2col" => {
                self.apply_preset("ultra-dense");
                self.variables.insert("columns".to_string(), "2".to_string());
            }
            "dense" => {
                self.variables.insert("fontsize".to_string(), "10pt".to_string());
                self.variables.insert("margin.x".to_string(), "2cm".to_string());
                self.variables.insert("margin.y".to_string(), "2cm".to_string());
            }
            "dense-2col" => {
                self.apply_preset("dense");
                self.variables.insert("columns".to_string(), "2".to_string());
            }
            "standard" => {
                self.variables.insert("fontsize".to_string(), "10pt".to_string());
                self.variables.insert("margin.x".to_string(), "2.5cm".to_string());
                self.variables.insert("margin.y".to_string(), "3cm".to_string());
            }
            "comfort" => {
                self.variables.insert("fontsize".to_string(), "12pt".to_string());
                self.variables.insert("margin.x".to_string(), "2.5cm".to_string());
                self.variables.insert("margin.y".to_string(), "3cm".to_string());
            }
            _ => {}
        }
    }

    pub fn set_pretty_code(&mut self) {
        let code_style = r#"
#show raw.where(block: false): box.with(fill: luma(240), inset: (x: 3pt), outset: (y: 3pt), radius: 3pt)

#show raw.where(block: true): it => {
  set text(size: 0.9em)
  align(center)[
    #block(fill: luma(250), inset: 1em, radius: 8pt)[#it]
  ]
}
"#;
        self.variables
            .insert("header-includes".to_string(), code_style.to_string());
    }

    pub fn override_variable(&mut self, key: &str, value: &str) {
        self.variables.insert(key.to_string(), value.to_string());
    }
}
