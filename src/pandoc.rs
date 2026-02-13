use std::process::{Command, Stdio};
use std::io::{self, Read, Write};
use anyhow::{Result, anyhow};
use which::which;
use crate::styles::Profile;
use tracing::{info, debug, error};

pub struct PandocWrapper;

impl PandocWrapper {
    pub fn convert(profile: &Profile, input: &str, output: &str, is_typst: bool) -> Result<()> {
        info!("Starting conversion: {} -> {}", input, output);

        // Enforce pandoc existence
        if which("pandoc").is_err() {
            return Err(anyhow!("Pandoc not found in system PATH. Please install Pandoc."));
        }

        let mut cmd = Command::new("pandoc");

        // Set input format to GFM
        cmd.arg("-f").arg("gfm");

        // Set input
        if input == "-" {
            cmd.stdin(Stdio::piped());
        } else {
            cmd.arg(input);
        }

        // Set output
        let actual_output = if output == "-" {
            if is_typst {
                "__quoin_temp.typ".to_string()
            } else {
                "__quoin_temp.pdf".to_string()
            }
        } else {
            output.to_string()
        };
        debug!("Pandoc output path set to: {}", actual_output);
        cmd.arg("-o").arg(&actual_output);

        // Set engine or output format
        if is_typst {
            cmd.arg("-t").arg("typst");
            cmd.arg("--standalone");
        } else {
            cmd.arg("--pdf-engine=typst");
        }

        // Create temporary metadata YAML file
        let metadata_path = format!("{}_metadata.yaml", actual_output);
        let header_path = format!("{}_header.typ", actual_output);
        let after_body_path = format!("{}_after_body.typ", actual_output);
        let lua_path = format!("{}_table.lua", actual_output);
        
        // Write Lua filter
        if profile.use_lua_table_filter {
            debug!("Writing Lua table filter to {}", lua_path);
            let lua_content = include_str!("assets/lua/table_dimensions.lua");
            std::fs::write(&lua_path, lua_content)?;
            cmd.arg("--lua-filter").arg(&lua_path);
        }

        // Serialize metadata to YAML
        debug!("Writing metadata to {}", metadata_path);
        let yaml_content = serde_yaml::to_string(&profile.metadata)?;
        std::fs::write(&metadata_path, yaml_content)?;
        cmd.arg("--metadata-file").arg(&metadata_path);

        // Write header includes to a separate file to prevent escaping
        if !profile.header_includes.is_empty() {
            debug!("Writing header includes to {}", header_path);
            let header_content = profile.header_includes.join("\n");
            std::fs::write(&header_path, header_content)?;
            cmd.arg("--include-in-header").arg(&header_path);
        }

        // Write after body includes to a separate file
        if !profile.after_body_includes.is_empty() {
            debug!("Writing after-body includes to {}", after_body_path);
            let after_body_content = profile.after_body_includes.join("\n");
            std::fs::write(&after_body_path, after_body_content)?;
            cmd.arg("--include-after-body").arg(&after_body_path);
        }

        debug!("Executing Pandoc: {:?}", cmd);
        let mut child = cmd.spawn().map_err(|e| {
            error!("Failed to spawn Pandoc process: {}", e);
            e
        })?;

        // If stdin is used, pipe current stdin to child
        if input == "-" {
            let mut stdin = child.stdin.take().ok_or_else(|| anyhow!("Failed to open stdin"))?;
            let mut buffer = Vec::new();
            io::stdin().read_to_end(&mut buffer)?;
            stdin.write_all(&buffer)?;
        }

        let status = child.wait().map_err(|e| {
            error!("Pandoc process wait failed: {}", e);
            e
        })?;
        
        // Cleanup temporary files
        debug!("Cleaning up temporary files...");
        let _ = std::fs::remove_file(&metadata_path);
        let _ = std::fs::remove_file(&header_path);
        let _ = std::fs::remove_file(&after_body_path);
        if profile.use_lua_table_filter {
            let _ = std::fs::remove_file(&lua_path);
        }

        if !status.success() {
            return Err(anyhow!("Pandoc execution failed with status: {}", status));
        }

        // If output was stdout, stream the temp file and then delete it
        if output == "-" {
            let mut file = std::fs::File::open(&actual_output)?;
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer)?;
            io::stdout().write_all(&buffer)?;
            std::fs::remove_file(&actual_output)?;
        }

        Ok(())
    }
}
