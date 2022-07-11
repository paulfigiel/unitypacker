use clap::{Parser, Subcommand};
use flate2::{write::GzEncoder, Compression};
use std::error::Error;
use std::fs::File;
use std::fs::{self};
use std::path::Path;
use unitypacker;

/// Create .unitypackages from command line
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Args {
    #[clap(subcommand)]
    command: Commands,
    /// Root of the unity project
    #[clap(short, long)]
    unity_project_root: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a .unitypackage
    Pack {
        /// Name of the .unitypackage to create
        #[clap(short, long)]
        package_name: String,
        /// Path to analyse
        #[clap(default_value_t = String::from("Assets"),short, long)]
        analysed_path: String,
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    match &args.command {
        Commands::Pack { package_name: name, analysed_path: path } => {
            let metas = unitypacker::find_unity_meta(path, args.unity_project_root)?;

            for ref meta in &metas {
                println!("Found Meta Entry : {:?}", meta.path);
            }

            let _ = fs::remove_dir_all(name);

            let unity_package_path = Path::new(name).with_extension("unitypackage");
            let tar_gz = File::create(&unity_package_path)?;
            let enc = GzEncoder::new(&tar_gz, Compression::default());
            let builder = &mut tar::Builder::new(enc);

            println!("Packing in {:?}", &unity_package_path);

            for ref mut meta in metas {
                meta.add_to_builder(builder)?;
            }
        }
    }

    println!("Done.");
    Ok(())
}
