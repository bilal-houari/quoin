use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Profile {
    pub variables: HashMap<String, String>,
    pub use_lua_table_filter: bool,
}

impl Profile {
    pub fn new() -> Self {
        let mut variables = HashMap::new();
        // Global defaults
        variables.insert("fontsize".to_string(), "10pt".to_string());
        variables.insert("lang".to_string(), "en".to_string());
        variables.insert("papersize".to_string(), "a4".to_string());

        Self { 
            variables,
            use_lua_table_filter: true,
        }
    }

    pub fn set_density(&mut self, density: &str) {
        match density.to_lowercase().as_str() {
            "ultra-dense" => {
                self.variables.insert("fontsize".to_string(), "8pt".to_string());
                self.variables.insert("margin.x".to_string(), "2cm".to_string());
                self.variables.insert("margin.y".to_string(), "2cm".to_string());
            }
            "dense" => {
                self.variables.insert("fontsize".to_string(), "10pt".to_string());
                self.variables.insert("margin.x".to_string(), "2cm".to_string());
                self.variables.insert("margin.y".to_string(), "2cm".to_string());
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
            _ => {
                // Default to standard if not matched
                self.variables.insert("fontsize".to_string(), "10pt".to_string());
                self.variables.insert("margin.x".to_string(), "2.5cm".to_string());
                self.variables.insert("margin.y".to_string(), "3cm".to_string());
            }
        }
    }

    pub fn set_two_cols(&mut self, enabled: bool) {
        let cols = if enabled { "2" } else { "1" };
        self.variables.insert("columns".to_string(), cols.to_string());
    }

    pub fn set_latex_font(&mut self) {
        self.variables.insert("mainfont".to_string(), "New Computer Modern".to_string());
    }

    pub fn set_alt_table(&mut self) {
        let table_style = r####"
#set table(stroke: 0.5pt + rgb("#888888"), inset: 0.5em)
#set table(
  fill: (_, y) => if y == 0 { rgb("#e4e4e4") },
)
"####;
        let current = self.variables.get("header-includes").cloned().unwrap_or_default();
        self.variables.insert("header-includes".to_string(), format!("{}\n{}", current, table_style));
    }

    pub fn set_pretty_code(&mut self) {
        let code_style = r####"
#show raw.where(block: false): box.with(fill: luma(240), inset: (x: 3pt), outset: (y: 3pt), radius: 3pt)

#show raw.where(block: true): it => {
  set text(size: 0.9em)
  align(center)[
    #block(fill: luma(250), inset: 1em, radius: 8pt)[#it]
  ]
}
"####;
        let current = self.variables.get("header-includes").cloned().unwrap_or_default();
        self.variables.insert("header-includes".to_string(), format!("{}\n{}", current, code_style));
    }

    pub fn override_variable(&mut self, key: &str, value: &str) {
        self.variables.insert(key.to_string(), value.to_string());
    }
}
