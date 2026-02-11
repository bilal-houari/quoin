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

        // Set input
        if input_path == "-" {
            cmd.stdin(Stdio::piped());
        } else {
            cmd.arg(input_path);
        }

        // Set output
        let actual_output = if output_path == "-" {
            "__quoin_temp.pdf".to_string()
        } else {
            output_path.to_string()
        };
        cmd.arg("-o").arg(&actual_output);

        // Set engine
        cmd.arg("--pdf-engine=typst");

        // Inject variables
        for (key, value) in &profile.variables {
            cmd.arg("-V").arg(format!("{}={}", key, value));
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
