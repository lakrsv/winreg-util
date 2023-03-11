use winreg_common::Key;
use std::collections::HashSet;
use std::path::PathBuf;
use std::process::Command;

#[derive(Debug, Clone)]
pub struct ExportError {
    msg: String,
}

impl ExportError {
    pub fn msg(&self) -> &str {
        &self.msg
    }
}

#[derive(Debug, Clone)]
pub struct ExportKey {
    root: Key,
    sub_keys: HashSet<String>,
}

impl ExportKey {
    pub fn new(root: Key, sub_keys: HashSet<String>) -> Self {
        ExportKey { root, sub_keys }
    }
}

pub fn export(keys: Vec<ExportKey>, output_dir: PathBuf) -> Result<(), ExportError> {
    if !cfg!(target_os = "windows") {
        panic!("The export function is only supported on Windows");
    }
    for key in keys {
        for sub_key in key.sub_keys {
            export_key(&key.root, sub_key, output_dir.clone())
                .unwrap_or_else(|e| println!("Failed exporting key. Error: {}", e.msg()));
        }
    }
    Ok(())
}

fn export_key(root: &Key, sub_key: String, mut output_dir: PathBuf) -> Result<(), ExportError> {
    let reg_key = format!("{}\\{}", root.get_name(), sub_key);

    output_dir.push(format!("{}-{}.dat", root.get_name(), sub_key));

    let command = format!(
        "reg save {} {}",
        reg_key,
        output_dir.as_path().to_str().unwrap()
    );

    let output = Command::new("cmd")
        .args(&["/C", &command])
        .output()
        .map_err(|err| ExportError {
            msg: err.to_string(),
        })?;
    if !output.status.success() {
        return Err(ExportError {
            msg: String::from_utf8(output.stderr).unwrap(),
        });
    }
    Ok(())
}
