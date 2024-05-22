// SPDX-FileCopyrightText: © 2024 Xronos Inc.
// SPDX-License-Identifier: BSD-3-Clause
use lfanalysis::ast::{self};
use lfbuildconfig::workspace_root;

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
    let cwd = workspace_root(&model.path);
    CommandDescription {
        cwd: cwd.to_str().unwrap().to_string(),
        command: command.to_string(),
        args,
    }
}
