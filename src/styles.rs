use serde::{Deserialize, Serialize};
use serde_yaml::{Mapping, Value};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Margin {
    pub x: String,
    pub y: String,
}

/// Core document metadata that translates directly to Pandoc/Typst variables.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Metadata {
    pub fontsize: String,
    pub lang: String,
    pub papersize: String,
    pub margin: Margin,
    pub columns: u8,
    pub mainfont: Option<String>,
    #[serde(rename = "section-numbering")]
    pub section_numbering: Option<String>,
    #[serde(flatten)]
    pub extra: Mapping,
}

#[derive(Debug, Clone, PartialEq)]
/// Represents a document conversion profile, holding all style and layout settings.
pub struct Profile {
    /// Variables passed to the Typst template via Pandoc metadata.
    pub metadata: Metadata,
    /// Raw Typst code snippets to be included in the document header.
    pub header_includes: Vec<String>,
    /// Raw Typst code snippets to be appended after the document body.
    pub after_body_includes: Vec<String>,
    /// Whether to use the custom Lua filter for better table dimension handling.
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
            section_numbering: None,
            extra: Mapping::new(),
        };

        Self { 
            metadata,
            header_includes: Vec::new(),
            after_body_includes: Vec::new(),
            use_lua_table_filter: true,
        }
    }

    /// Set the layout density by adjusting font size and margins.
    /// 
    /// Supported levels: "ultra-dense", "dense", "standard", "comfort".
    pub fn set_density(&mut self, level: &str) {
        tracing::debug!("Setting density to {}", level);
        match level.to_lowercase().as_str() {
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
        tracing::debug!("Setting two columns: {}", enabled);
        if enabled {
            self.metadata.columns = 2;
        } else {
            self.metadata.columns = 1;
        }
    }

    pub fn set_latex_font(&mut self) {
        tracing::debug!("Enabling LaTeX-style font");
        self.metadata.mainfont = Some("NewComputerModern08".to_string());
    }

    pub fn set_global_defaults(&mut self) {
        let defaults = include_str!("assets/typst/defaults.typ");
        self.header_includes.push(defaults.to_string());
    }

    pub fn set_alt_table(&mut self) {
        tracing::debug!("Enabling alternative table styling");
        let table_style = include_str!("assets/typst/alt_table.typ");
        self.header_includes.push(table_style.to_string());
    }

    pub fn set_pretty_code(&mut self) {
        tracing::debug!("Enabling pretty code blocks");
        let code_style = include_str!("assets/typst/pretty_code.typ");
        self.header_includes.push(code_style.to_string());
    }

    pub fn set_section_numbering(&mut self, enabled: bool) {
        tracing::debug!("Setting section numbering: {}", enabled);
        self.metadata.section_numbering = if enabled {
            Some("1.1".to_string())
        } else {
            None
        };
    }

    pub fn set_outline(&mut self) {
        tracing::debug!("Enabling document outline (TOC)");
        let outline_style = include_str!("assets/typst/outline.typ");
        self.after_body_includes.push(outline_style.to_string());
    }

    pub fn override_variable(&mut self, key: &str, value: &str) {
        tracing::debug!("Overriding variable {} = {}", key, value);
        // Attempt to set structured fields first
        match key {
            "fontsize" => self.metadata.fontsize = value.to_string(),
            "lang" => self.metadata.lang = value.to_string(),
            "papersize" => self.metadata.papersize = value.to_string(),
            "margin.x" => self.metadata.margin.x = value.to_string(),
            "margin.y" => self.metadata.margin.y = value.to_string(),
            "columns" => if let Ok(n) = value.parse() { self.metadata.columns = n },
            "mainfont" => self.metadata.mainfont = Some(value.to_string()),
            "section-numbering" | "sectionnumbering" => self.metadata.section_numbering = Some(value.to_string()),
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
