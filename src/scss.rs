
use crate::prelude::*;
use std::{fs, path::PathBuf};


/// Compiles a SCSS file to CSS.
fn compile_scss_file(path: PathBuf) -> Result<(), std::io::Error> {    

    let options = grass::Options::default()
        .unicode_error_messages(true)
        .style(grass::OutputStyle::Compressed);

    match grass::from_path(&path, &options) {
        Ok(css) => {
            fs::write(path.with_extension("css"), css)?;
        },
        Err(e) => {
            eprintln!("Failed to compile SCSS: {}", e);            
        }
    }

    Ok(())
}

/// Compiles all SCSS files in the style directory.
pub fn compile_all_scss() {
    let path = PathBuf::new()
        .join(RES_PATH)
        .join("style");
    
    let walker = walkdir::WalkDir::new(path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| {
            !e.file_type().is_dir() 
            && e.path().extension().map_or(false, |ext| ext == "scss")
        });

    for entry in walker {
        compile_scss_file(entry.path().to_path_buf()).unwrap();
    }
}