use std::collections::HashSet;
use std::path::Path;
use std::{env, fs, io};
use winreg_util::export::{export, ExportKey};
use winreg_util::key::Key::{HkeyClassesRoot, HkeyLocalMachine, HkeyUsers};

fn main() -> io::Result<()> {
    export_test()
}

fn export_test() -> io::Result<()> {
    let mut export_path = env::current_dir()?;
    export_path.push(Path::new("export"));
    fs::create_dir_all(&export_path).expect(&*format!(
        "Failed creating export directory: {:?}",
        export_path
    ));

    let keys = vec![
        ExportKey::new(HkeyLocalMachine, HashSet::from([String::from("SOFTWARE")])),
        ExportKey::new(HkeyUsers, HashSet::from([String::from("SOFTWARE")])),
        ExportKey::new(HkeyClassesRoot, HashSet::from([String::from("SOFTWARE")])),
    ];
    export(keys, export_path).expect("Failed to export registry keys");
    Ok(())
}
