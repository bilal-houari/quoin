use quoin::pandoc::PandocWrapper;
use quoin::styles::Profile;
use std::fs;
use std::path::Path;

#[test]
fn test_all_styles_and_generate_outputs() {
    let samples_dir = "tests/samples";
    let output_dir = "test_output";
    let styles = vec![
        "ultra-dense",
        "ultra-dense-2col",
        "dense",
        "dense-2col",
        "standard",
        "comfort",
    ];

    // Ensure output directory exists
    let _ = fs::remove_dir_all(output_dir);
    fs::create_dir_all(output_dir).expect("Failed to create output directory");

    let entries = fs::read_dir(samples_dir).expect("Failed to read samples directory");

    for entry in entries {
        let entry = entry.expect("Failed to read entry");
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("md") {
            let base_name = path.file_stem().unwrap().to_str().unwrap();
            println!("Processing sample: {}", base_name);

            for style in &styles {
                println!("  Applying style: {}", style);

                let mut profile = Profile::new();
                profile.apply_preset(style);

                // Generate PDF
                let pdf_output = format!("{}/{}_{}.pdf", output_dir, base_name, style);
                PandocWrapper::convert(&profile, path.to_str().unwrap(), &pdf_output)
                    .expect(&format!("Failed to generate PDF for style {}", style));

                // Generate Typst source
                let typ_output = format!("{}/{}_{}.typ", output_dir, base_name, style);
                PandocWrapper::convert(&profile, path.to_str().unwrap(), &typ_output)
                    .expect(&format!("Failed to generate Typst for style {}", style));

                assert!(Path::new(&pdf_output).exists());
                assert!(Path::new(&typ_output).exists());
            }

            // Test modifier: comfort + pretty-code
            let mut profile = Profile::new();
            profile.apply_preset("comfort");
            profile.set_pretty_code();
            let comb_name = "comfort_pretty-code";
            
            let pdf_output = format!("{}/{}_{}.pdf", output_dir, base_name, comb_name);
            PandocWrapper::convert(&profile, path.to_str().unwrap(), &pdf_output)
                .expect("Failed to generate PDF for combination");

            let typ_output = format!("{}/{}_{}.typ", output_dir, base_name, comb_name);
            PandocWrapper::convert(&profile, path.to_str().unwrap(), &typ_output)
                .expect("Failed to generate Typst for combination");

            assert!(Path::new(&pdf_output).exists());
            assert!(Path::new(&typ_output).exists());
        }
    }
}
