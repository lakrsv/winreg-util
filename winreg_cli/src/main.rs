use clap::Parser;
use std::path::PathBuf;
use std::{fs, io};
use winreg_cli::{Cli, Commands, ExportArgs, InterrogateArgs};
use winreg_export::export::export;

fn main() -> io::Result<()> {
    let args = Cli::parse();
    match args.command {
        Commands::Export(args) => run_export(args),
        Commands::Interrogate(args) => run_interrogate(args),
    }
}

fn run_export(args: ExportArgs) -> io::Result<()> {
    let export_path = PathBuf::from(args.get_output_path());
    let export_keys = args.build_export_keys();

    if export_path.is_dir() {
        eprintln!("The directory {} already exists", &export_path.display());
        return Ok(());
    }

    fs::create_dir_all(&export_path).expect(&*format!(
        "Failed creating export directory: {}",
        &export_path.display()
    ));

    println!("Exporting registry keys to {}", &export_path.display());

    match export(export_keys, PathBuf::from(export_path)) {
        Ok(_) => {
            println!("Export completed")
        }
        Err(e) => {
            eprintln!("Error exporting registry keys: {}", e.msg());
        }
    }

    println!("Export completed");
    Ok(())
}

fn run_interrogate(args: InterrogateArgs) -> io::Result<()> {
    todo!()
}
