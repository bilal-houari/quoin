use serde::Serialize;
use serde_yaml::{Mapping, Value};

#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct Margin {
    pub x: String,
    pub y: String,
}

#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct Metadata {
    pub fontsize: String,
    pub lang: String,
    pub papersize: String,
    pub margin: Margin,
    pub columns: u8,
    pub mainfont: Option<String>,
    #[serde(flatten)]
    pub extra: Mapping,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Profile {
    pub metadata: Metadata,
    pub header_includes: Vec<String>,
    pub use_lua_table_filter: bool,
}

impl Profile {
    pub fn new() -> Self {
        let metadata = Metadata {
            fontsize: "10pt".to_string(),
            lang: "en".to_string(),
            papersize: "a4".to_string(),
            margin: Margin {
                x: "2.5cm".to_string(),
                y: "3cm".to_string(),
            },
            columns: 1,
            mainfont: None,
            extra: Mapping::new(),
        };

        Self { 
            metadata,
            header_includes: Vec::new(),
            use_lua_table_filter: true,
        }
    }

    pub fn set_density(&mut self, density: &str) {
        match density.to_lowercase().as_str() {
            "ultra-dense" => {
                self.metadata.fontsize = "8pt".to_string();
                self.metadata.margin.x = "2cm".to_string();
                self.metadata.margin.y = "2cm".to_string();
            }
            "dense" => {
                self.metadata.fontsize = "10pt".to_string();
                self.metadata.margin.x = "2cm".to_string();
                self.metadata.margin.y = "2cm".to_string();
            }
            "standard" => {
                self.metadata.fontsize = "10pt".to_string();
                self.metadata.margin.x = "2.5cm".to_string();
                self.metadata.margin.y = "3cm".to_string();
            }
            "comfort" => {
                self.metadata.fontsize = "12pt".to_string();
                self.metadata.margin.x = "2.5cm".to_string();
                self.metadata.margin.y = "3cm".to_string();
            }
            _ => {
                // Default to standard if not matched
                self.metadata.fontsize = "10pt".to_string();
                self.metadata.margin.x = "2.5cm".to_string();
                self.metadata.margin.y = "3cm".to_string();
            }
        }
    }

    pub fn set_two_cols(&mut self, enabled: bool) {
        self.metadata.columns = if enabled { 2 } else { 1 };
    }

    pub fn set_latex_font(&mut self) {
        self.metadata.mainfont = Some("New Computer Modern".to_string());
    }

    pub fn set_global_defaults(&mut self) {
        let defaults = include_str!("assets/typst/defaults.typ");
        self.header_includes.push(defaults.to_string());
    }

    pub fn set_alt_table(&mut self) {
        let table_style = include_str!("assets/typst/alt_table.typ");
        self.header_includes.push(table_style.to_string());
    }

    pub fn set_pretty_code(&mut self) {
        let code_style = include_str!("assets/typst/pretty_code.typ");
        self.header_includes.push(code_style.to_string());
    }

    pub fn override_variable(&mut self, key: &str, value: &str) {
        // Attempt to set structured fields first
        match key {
            "fontsize" => self.metadata.fontsize = value.to_string(),
            "lang" => self.metadata.lang = value.to_string(),
            "papersize" => self.metadata.papersize = value.to_string(),
            "margin.x" => self.metadata.margin.x = value.to_string(),
            "margin.y" => self.metadata.margin.y = value.to_string(),
            "columns" => if let Ok(n) = value.parse() { self.metadata.columns = n },
            "mainfont" => self.metadata.mainfont = Some(value.to_string()),
            _ => {
                // Support dotted keys for nesting in extra
                let parts: Vec<&str> = key.split('.').collect();
                let mut current = &mut self.metadata.extra;
                
                for i in 0..parts.len() {
                    let k = Value::String(parts[i].to_string());
                    if i == parts.len() - 1 {
                        current.insert(k, Value::String(value.to_string()));
                    } else {
                        if !current.contains_key(&k) || !current.get(&k).unwrap().is_mapping() {
                            current.insert(k.clone(), Value::Mapping(Mapping::new()));
                        }
                        // Re-fetch to satisfy borrow checker
                        current = current.get_mut(&k).unwrap().as_mapping_mut().unwrap();
                    }
                }
            }
        }
    }
}
