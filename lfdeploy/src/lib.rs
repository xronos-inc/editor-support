use std::{path::PathBuf, process::Command};

use lfanalysis::ast::{self, LfFile};

pub struct CommandDescription {
    pub cwd: String,
    pub command: String,
    pub args: Vec<String>,
}

pub fn deploy_command(cst: &str) -> CommandDescription {
    let model = ast::parse(cst);
    let command = "./xronos-docker.sh";
    let basename = model.path.file_stem().unwrap().to_str().unwrap();
    let args = lfanalysis::federate_names(cst)
        .iter()
        .flat_map(|federate| {
            vec![
                federate.clone(),
                format!(
                    "fed-gen/{}/src-gen/federate__{}/Dockerfile",
                    basename, federate
                ),
            ]
        })
        .collect();
    let cwd = package_root(&model);
    CommandDescription {
        cwd: cwd.to_str().unwrap().to_string(),
        command: command.to_string(),
        args,
    }
}

pub fn package_root(f: &LfFile) -> PathBuf {
    let mut cwd = f.path.clone();
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
                    .path
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
        let f = LfFile {
            path: PathBuf::from("/home/peter/lingua-franca/test/C/src/docker/federated/DistributedCountContainerized.lf"),
            model: ast::Model {
                target: ast::Target {

                },
                imports: vec![],
                preambles: vec![],
                reactors: vec![],
            },
        };
        assert_eq!(
            package_root(&f),
            PathBuf::from("/home/peter/lingua-franca/test/C")
        );
    }
}
