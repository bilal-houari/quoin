use quoin::pandoc::PandocWrapper;
use quoin::styles::Profile;
use std::fs;
use std::path::Path;

fn setup_output_dir(dir: &str) {
    let _ = fs::create_dir_all(dir);
}

#[test]
fn test_programmatic_conversion() {
    let sample_path = "tests/samples/basic.md";
    let output_dir = "test_output/programmatic";
    setup_output_dir(output_dir);
    
    let mut profile = Profile::new();
    profile.set_global_defaults();
    profile.set_density("standard");
    
    let pdf_output = format!("{}/basic.pdf", output_dir);
    let typ_output = format!("{}/basic.typ", output_dir);

    // Test PDF generation
    PandocWrapper::convert(&profile, sample_path, &pdf_output)
        .expect("Failed to generate PDF");
    assert!(Path::new(&pdf_output).exists());

    // Test Typst source generation
    PandocWrapper::convert(&profile, sample_path, &typ_output)
        .expect("Failed to generate Typst source");
    assert!(Path::new(&typ_output).exists());
}

#[test]
fn test_yaml_header_override() {
    let sample_path = "tests/samples/override.md";
    let output_dir = "test_output/programmatic";
    setup_output_dir(output_dir);
    let typ_output = format!("{}/override_test.typ", output_dir);
    
    let mut profile = Profile::new();
    profile.metadata.lang = "en".to_string(); // Profile says English
    
    PandocWrapper::convert(&profile, sample_path, &typ_output)
        .expect("Failed override test conversion");
    
    let content = fs::read_to_string(&typ_output).unwrap();
    // In override.md, we set lang: fr. It should override the Profile's "en".
    assert!(content.contains("lang: \"fr\""));
}

#[test]
fn test_nested_metadata_conversion_success() {
    let sample_path = "tests/samples/basic.md";
    let output_dir = "test_output/programmatic";
    setup_output_dir(output_dir);
    let typ_output = format!("{}/nested_test.typ", output_dir);
    
    let mut profile = Profile::new();
    profile.override_variable("custom.nested.key", "nested_value");
    
    // Just verify that the conversion succeeds with nested metadata
    PandocWrapper::convert(&profile, sample_path, &typ_output)
        .expect("Failed nested metadata conversion");
}
