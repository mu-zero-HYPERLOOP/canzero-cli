use std::path::PathBuf;

use can_c_codegen_rs::options::Options;

use crate::errors::Result;

pub fn command_gen(node_name : &str, output_dir : &str) -> Result<()>{
    let config = can_live_config_rs::fetch_live_config()?;

    let mut options = Options::default();
    // chekc if the output_dir exists!
    
    let output_dir_path : PathBuf = output_dir.into();
    if !output_dir_path.is_dir() {
        std::fs::create_dir(output_dir_path)?;
    }

    options.set_source_file_path(&format!("{output_dir}/{}.c", options.namespace()));
    options.set_header_file_path(&format!("{output_dir}/{}.h", options.namespace()));

    can_c_codegen_rs::generate(node_name, config, options)?;

    Ok(())
}
