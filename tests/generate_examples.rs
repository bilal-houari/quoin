use quoin::pandoc::PandocWrapper;
use quoin::styles::Profile;
use std::fs;

#[test]
fn generate_showcase_examples() {
    let sample_path = "tests/samples/advanced.md";
    let output_dir = "test_output/examples";
    
    // Ensure output directory exists
    let _ = fs::create_dir_all(output_dir);

    // 1. Standard (Default)
    {
        let mut profile = Profile::new();
        profile.set_global_defaults();
        profile.set_density("standard");
        profile.set_alt_table();
        profile.set_pretty_code();
        
        PandocWrapper::convert(&profile, sample_path, &format!("{}/standard.pdf", output_dir)).unwrap();
        PandocWrapper::convert(&profile, sample_path, &format!("{}/standard.typ", output_dir)).unwrap();
    }

    // 2. Ultra-Dense
    {
        let mut profile = Profile::new();
        profile.set_global_defaults();
        profile.set_density("ultra-dense");
        profile.set_alt_table();
        profile.set_pretty_code();
        
        PandocWrapper::convert(&profile, sample_path, &format!("{}/ultra_dense.pdf", output_dir)).unwrap();
    }

    // 3. Two Columns
    {
        let mut profile = Profile::new();
        profile.set_global_defaults();
        profile.set_two_cols(true);
        profile.set_alt_table();
        profile.set_pretty_code();
        
        PandocWrapper::convert(&profile, sample_path, &format!("{}/two_columns.pdf", output_dir)).unwrap();
    }

    // 4. LaTeX Font (New Computer Modern)
    {
        let mut profile = Profile::new();
        profile.set_global_defaults();
        profile.set_latex_font();
        profile.set_alt_table();
        profile.set_pretty_code();
        
        PandocWrapper::convert(&profile, sample_path, &format!("{}/latex_font.pdf", output_dir)).unwrap();
    }
}
