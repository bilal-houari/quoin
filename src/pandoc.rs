use std::process::{Command, Stdio};
use std::io::{self, Read, Write};
use anyhow::{Result, anyhow};
use which::which;
use crate::styles::Profile;

pub struct PandocWrapper;

impl PandocWrapper {
    pub fn convert(profile: &Profile, input_path: &str, output_path: &str) -> Result<()> {
        // Enforce pandoc existence
        if which("pandoc").is_err() {
            return Err(anyhow!("Pandoc not found in system PATH. Please install Pandoc."));
        }

        let mut cmd = Command::new("pandoc");

        // Set input format to GFM
        cmd.arg("-f").arg("gfm");

        // Set input
        if input_path == "-" {
            cmd.stdin(Stdio::piped());
        } else {
            cmd.arg(input_path);
        }

        // Set output
        let is_typst = output_path.ends_with(".typ");
        let actual_output = if output_path == "-" {
            "__quoin_temp.pdf".to_string()
        } else {
            output_path.to_string()
        };
        cmd.arg("-o").arg(&actual_output);

        // Set engine or output format
        if is_typst {
            cmd.arg("-t").arg("typst");
            cmd.arg("--standalone");
        } else {
            cmd.arg("--pdf-engine=typst");
        }

        // Create temporary metadata JSON file
        let metadata_path = format!("{}_metadata.json", actual_output);
        let header_path = format!("{}_header.typ", actual_output);
        let mut has_header = false;
        
        let mut metadata_map = serde_json::Map::new();
        for (key, value) in &profile.variables {
            if key == "header-includes" {
                std::fs::write(&header_path, value)?;
                has_header = true;
                continue;
            }

            // Check for nested keys (e.g. margin.x)
            if key.contains('.') {
                let parts: Vec<&str> = key.split('.').collect();
                let parent = parts[0];
                let child = parts[1];
                
                let entry = metadata_map.entry(parent.to_string())
                    .or_insert(serde_json::Value::Object(serde_json::Map::new()));
                
                if let serde_json::Value::Object(map) = entry {
                    map.insert(child.to_string(), serde_json::Value::String(value.clone()));
                }
            } else {
                metadata_map.insert(key.clone(), serde_json::Value::String(value.clone()));
            }
        }
        let metadata_json = serde_json::Value::Object(metadata_map);
        std::fs::write(&metadata_path, serde_json::to_string_pretty(&metadata_json)?)?;
        
        cmd.arg("--metadata-file").arg(&metadata_path);
        if has_header {
            cmd.arg("--include-in-header").arg(&header_path);
        }

        let mut child = cmd.spawn()?;

        // If stdin is used, pipe current stdin to child
        if input_path == "-" {
            let mut stdin = child.stdin.take().ok_or_else(|| anyhow!("Failed to open stdin"))?;
            let mut buffer = Vec::new();
            io::stdin().read_to_end(&mut buffer)?;
            stdin.write_all(&buffer)?;
        }

        let status = child.wait()?;
        
        // Cleanup temporary files
        let _ = std::fs::remove_file(&metadata_path);
        if has_header {
            let _ = std::fs::remove_file(&header_path);
        }

        if !status.success() {
            return Err(anyhow!("Pandoc execution failed with status: {}", status));
        }

        // If output was stdout, stream the temp file and then delete it
        if output_path == "-" {
            let mut file = std::fs::File::open(&actual_output)?;
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer)?;
            io::stdout().write_all(&buffer)?;
            std::fs::remove_file(&actual_output)?;
        }

        Ok(())
    }
}
