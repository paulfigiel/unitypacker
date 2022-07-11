use clap::Parser;
use std::error::Error;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use tar::Header;
use walkdir::WalkDir;
use yaml_rust::YamlLoader;

/// Create .unitypackages from command line
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the .unitypackage to create
    #[clap(short, long)]
    name: String,
    /// Path to analyse
    #[clap(short, long)]
    path: String,
    /// Root of the unity project
    #[clap(short, long)]
    unity_project_root: String,
}

#[derive(Debug)]
pub struct UnityMeta {
    pub guid: String,
    pub path: PathBuf,
    pub asset_file: Option<PathBuf>,
    pub meta_file: PathBuf,
}

impl UnityMeta {
    pub fn add_to_builder<T: Write>(
        &self,
        builder: &mut tar::Builder<T>,
    ) -> Result<(), Box<dyn Error>> {
        let new_asset_path = Path::new(&self.guid).join("asset");
        let new_meta_path = Path::new(&self.guid).join("asset.meta");
        let pathname_path = Path::new(&self.guid).join("pathname");

        let char_count = self.path.to_str().unwrap().chars().count() as u64;
        let mut header = Header::new_gnu();
        header.set_size(char_count);
        header.set_cksum();

        builder.append_data(
            &mut header,
            &pathname_path,
            self.path.to_str().unwrap().as_bytes(),
        )?;

        {
            let mut meta_file = File::open(&self.meta_file)?;
            builder.append_file(new_meta_path, &mut meta_file)?;
        }

        if let Some(ref asset) = self.asset_file {
            let mut asset_file = File::open(asset)?;
            builder.append_file(new_asset_path, &mut asset_file)?;
        }

        Ok(())
    }
}

pub fn find_unity_meta(
    path: &String,
    unity_root: Option<String>,
) -> Result<Vec<UnityMeta>, Box<dyn Error>> {
    let mut res = vec![];

    for entry in WalkDir::new(path) {
        if let Ok(e) = entry {
            if let Some(ext) = e.path().extension() {
                if ext == "meta" {
                    let content = fs::read_to_string(e.path())?;
                    let content = content.trim_start_matches("\u{feff}");
                    let docs = YamlLoader::load_from_str(&content)?;
                    let asset_path = e.path().with_extension("");

                    let mut path = e.path().with_extension("");
                    if let Some(ref root) = unity_root {
                        path = path
                            .strip_prefix(root)
                            .expect(
                                format!(
                                    "Analysed path {:?} is conflicting with unity root {:?}",
                                    path, root
                                )
                                .as_str(),
                            )
                            .to_path_buf();
                    }

                    res.push(UnityMeta {
                        guid: docs[0]["guid"].as_str().unwrap().to_owned(),
                        asset_file: if asset_path.is_file() {
                            Some(asset_path)
                        } else {
                            None
                        },
                        meta_file: e.path().to_path_buf(),
                        path: path,
                    });
                }
            }
        }
    }
    return Ok(res);
}
