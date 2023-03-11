use clap::{Args, Parser, Subcommand};
use itertools::Itertools;
use std::collections::HashSet;
use winreg_export::export::ExportKey;
use winreg_export::key::Key;
/// Program used to export or interrogate registry hive files
#[derive(Parser, Clone, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    /// The operation to execute
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Clone, Debug)]
pub enum Commands {
    /// Export registry hive files
    Export(ExportArgs),
    /// Interrogate a registry hive with various predicates
    Interrogate(InterrogateArgs),
}

#[derive(Args, Debug, Clone)]
pub struct ExportArgs {
    /// Directory to export the registry hive to
    #[arg(short, long)]
    output_path: String,
    /// The keys to export
    #[arg(short, long)]
    keys: Vec<String>,
}

#[derive(Args, Debug, Clone)]
pub struct InterrogateArgs {}

impl ExportArgs {
    pub fn get_output_path(&self) -> &str {
        &self.output_path
    }

    pub fn build_export_keys(&self) -> Vec<ExportKey> {
        let keys: Vec<ExportKey> = self
            .keys
            .iter()
            .filter_map(|key| match Key::try_from(key.as_str()) {
                Ok(root) => {
                    let sub_key = &key[root.get_name().len() + 2..];
                    return Some((root, sub_key));
                }
                Err(e) => {
                    eprintln!("Failed parsing key: {}", e.msg());
                    return None;
                }
            })
            .into_group_map()
            .into_iter()
            .map(|grouped| {
                ExportKey::new(
                    grouped.0,
                    HashSet::from_iter(
                        grouped
                            .1
                            .into_iter()
                            .map(|s| s.to_string())
                            .collect::<Vec<String>>(),
                    ),
                )
            })
            .collect();
        keys
    }
}
