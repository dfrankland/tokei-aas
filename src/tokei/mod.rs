mod languages;

use self::languages::convert;
pub use self::languages::Languages;
use anyhow::{anyhow, Result};
use std::process::Command;
use tempdir::TempDir;
use tokei::{Config as TokeiConfig, Languages as TokeiLanguages};

pub async fn tokei(
    repo: String,
    paths: Option<Vec<String>>,
    ignored: Option<Vec<String>>,
) -> Result<Languages> {
    let tmp_dir = TempDir::new("tokei")
        .map_err(|_| anyhow!("Could not create temporary directory for git clone for tokei job"))?;
    let tmp_dir_path = tmp_dir.path();

    async {
        let mut git = Command::new("git");
        git.args(
            [
                "clone",
                "--depth",
                "1",
                &repo,
                tmp_dir_path.to_str().unwrap(),
            ]
            .iter(),
        );

        let exit_status = git
            .status()
            .map_err(|_| anyhow!("Failed to complete git clone for tokei job"))?;

        if !exit_status.success() {
            return Err(anyhow!("Git clone for tokei job was not successful"));
        }

        Ok(())
    }
    .await?;

    let paths_with_defaults = if let Some(paths) = paths {
        if paths.is_empty() {
            vec![String::from("")]
        } else {
            paths
        }
    } else {
        vec![String::from("")]
    };

    let mut full_paths = vec![];
    for p in paths_with_defaults.iter() {
        if let Some(path_string) = tmp_dir_path.join(p).to_str() {
            full_paths.push(String::from(path_string));
        } else {
            return Err(anyhow!("Path `{}` is not good", p));
        }
    }

    let mut languages = TokeiLanguages::new();
    languages.get_statistics(
        &full_paths,
        &ignored
            .unwrap_or_else(|| vec![])
            .iter()
            .map(|s| &**s)
            .collect::<Vec<&str>>(),
        &TokeiConfig::default(),
    );

    Ok(convert(languages))
}
