use std::{
    io,
    path::{Path, PathBuf},
};

use liblingo::{backends::lfc::LfcJsonArgs, package::ConfigFile};

pub fn lfc_json(lf_abspath: &Path) -> io::Result<Option<String>> {
    let lingo_toml = lingo_path(lf_abspath);
    let root = workspace_root(lf_abspath);
    let config: ConfigFile = ConfigFile::from(&lingo_toml)?;
    let config = config.to_config(&root);
    let app = config
        .apps
        .iter()
        .find(|app| app.main_reactor == lf_abspath);
    app.map(|app| Ok(LfcJsonArgs::new(app, true).to_string()))
        .transpose()
}

fn lingo_path(lf_file: &Path) -> PathBuf {
    workspace_root(lf_file).join("Lingo.toml")
}

pub fn workspace_root(f: &Path) -> PathBuf {
    let mut cwd = f.to_path_buf();
    loop {
        let is_src = if let Some(fname) = cwd.file_name() {
            fname == "src"
        } else {
            false
        };
        match cwd.parent() {
            None => {
                // TODO: return Result?
                return f
                    .parent()
                    .expect("an LfFile should never have an empty path")
                    .to_path_buf();
            }
            Some(parent) => {
                cwd = parent.to_path_buf();
                if is_src {
                    return cwd;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_package_root() {
        let f = PathBuf::from("/home/username/lingua-franca/test/C/src/docker/federated/DistributedCountContainerized.lf");
        assert_eq!(
            workspace_root(&f),
            PathBuf::from("/home/username/lingua-franca/test/C")
        );
    }
}
