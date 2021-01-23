use std::{error::Error, process::Command};

#[cfg(target_os = "macos")]
fn main() -> Result<(), Box<dyn Error>> {
    Ok(())
}

#[cfg(not(target_os = "macos"))]
enum ShaderType {
    Vertex,
    Fragment,
}

// didnt try to compile shaders on macos
#[cfg(not(target_os = "macos"))]
fn main() -> Result<(), Box<dyn Error>> {
    // Tell the build script to only run again if we change our source shaders
    // println!("cargo:rerun-if-changed=src/shaders");
    println!("cargo:rerun-if-changed=src/shaders");

    for entry in std::fs::read_dir("src/shaders")? {
        let entry = entry?;

        if entry.file_type()?.is_file() {
            let in_path = entry.path();

            // Support only vertex and fragment shaders currently
            let shader_type =
                in_path
                    .extension()
                    .and_then(|ext| match ext.to_string_lossy().as_ref() {
                        "vert" => Some(ShaderType::Vertex),
                        "frag" => Some(ShaderType::Fragment),
                        _ => None,
                    });
            if let Some(_shader_type) = shader_type {
                let path_str = format!("{}.spv", in_path.display());
                let dest = std::path::Path::new(&path_str);

                let command = format!(
                    ".\\glslangValidator.exe -V {}  -o {}",
                    in_path.display(),
                    dest.display()
                );
                if dest.exists() {
                    std::fs::remove_file(&dest)?;
                }
                println!("command {}", command);
                Command::new("cmd")
                    .args(&["/C", &command])
                    .output()
                    .expect("failed to execute process");
            }
        }
    }
    Ok(())
}
