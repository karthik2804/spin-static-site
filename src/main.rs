use std::{
    env,
    fs::{self, File},
    path::{PathBuf, Path},
    process::Command,
};

use anyhow::{bail, Error, Result};
use clap::{FromArgMatches, Parser};
use serde_json::{json, Value};
use tempfile::tempdir;

const DEPLOY_META_FILENAME: &str = ".spin-static-site-deploy";
const SPIN_BIN_PATH: &str = "SPIN_BIN_PATH";

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the site to be deployed
    name: Option<String>,
    #[clap(short = 'd', long, value_parser, default_value = "./")]
    directory: PathBuf,
}

fn main() -> Result<(), Error> {
    let app = Args::clap();
    let matches = app.get_matches();
    let cli = Args::from_arg_matches(&matches)?;

    let temp_dir = tempdir()?.into_path();
    let work_dir = env::current_dir()?.join(cli.directory);

    let app_name: String = match cli.name {
        Some(val) => val,
        None => match get_deploy_name(&work_dir) {
            Ok(val) => val,
            Err(_) => bail!("Need to specify app name"),
        },
    };

    let spin = std::env::var(SPIN_BIN_PATH)?;
    // Create a new spin app in the temp dir
    env::set_current_dir(&temp_dir)?;
    Command::new(&spin)
        .arg("new")
        .arg("static-fileserver")
        .arg(&app_name)
        .arg("--value")
        .arg("http-path=/...")
        .arg("--accept-defaults")
        .status()?;

    env::set_current_dir(&temp_dir.join(&app_name))?;

    fs::create_dir("assets")?;
    println!("Creating app");
    for entry in recurse(&work_dir, None) {
        println!("{:?}", entry);
        let dest = &temp_dir.join(&app_name).join("assets").join(&entry);
        create_directory_and_parents_if_not_exist(dest.to_str().unwrap_or_default())?;
        fs::copy(work_dir.join(&entry), dest)?;
    }

    println!("Deploying");
    let output = Command::new(spin).arg("deploy").status()?;
    if output.success() {
        write_deploy_meta(&work_dir, &app_name)?;
    }

    Ok(())
}

fn recurse(path: &PathBuf, root: Option<PathBuf>) -> Vec<PathBuf> {
    let Ok(entries) = fs::read_dir(path) else { return vec![] };
    let root = match root {
        Some(val) => val,
        None => path.clone()
    };
    entries
        .flatten()
        .flat_map(|entry| {
            let Ok(meta) = entry.metadata() else { return vec![] };
            if meta.is_dir() {
                return recurse(&entry.path(), Some(root.clone()));
            }
            if meta.is_file() {
                return vec![entry.path().strip_prefix(root.clone()).unwrap().to_path_buf()];
            }
            vec![]
        })
        .collect()
}

fn get_deploy_name(dir: &PathBuf) -> Result<String, Error> {
    let deploy_data = fs::read_to_string(dir.join(DEPLOY_META_FILENAME));
    match deploy_data {
        Ok(val) => {
            let meta: Value = serde_json::from_str(&val)?;
            Ok(meta["name"].as_str().unwrap_or_default().to_owned())
        }
        Err(_) => {
            bail!("error reading file")
        }
    }
}

fn write_deploy_meta(dir: &PathBuf, name: &str) -> Result<(), Error> {
    let data = json!({
        "name": name
    });
    let mut file = File::create(dir.join(DEPLOY_META_FILENAME))?;
    serde_json::to_writer(&mut file, &data)?;
    println!(
        "Deploy metadata successfully written to {}",
        dir.join(DEPLOY_META_FILENAME).to_str().unwrap_or_default()
    );
    return Ok(());
}

fn create_directory_and_parents_if_not_exist(path: &str) -> Result<(), std::io::Error> {
    let parent_dir = Path::new(path).parent().ok_or_else(|| {
        std::io::Error::new(
            std::io::ErrorKind::Other,
            "Invalid path, cannot determine parent directory.",
        )
    })?;
    
    if !parent_dir.exists() {
        fs::create_dir_all(parent_dir)?;
    }

    Ok(())
}

