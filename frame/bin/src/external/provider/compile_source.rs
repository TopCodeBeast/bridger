use cargo_util::ProcessBuilder;
use colored::Colorize;
use std::path::PathBuf;

use support_common::error::BridgerError;

use crate::external;
use crate::external::execute::ISubcommandExecutor;
use crate::external::types::CompileChannel;

/// Compile source code and execute binary
#[derive(Clone, Debug)]
pub struct CompileSourceExecutor {
    command: String,
    args: Vec<String>,
    channel: CompileChannel,
    toolchain_version: Option<String>,
}

impl CompileSourceExecutor {
    pub fn new(
        command: String,
        args: Vec<String>,
        channel: CompileChannel,
        toolchain_version: Option<String>,
    ) -> Self {
        Self {
            command,
            args,
            channel,
            toolchain_version,
        }
    }
}

impl ISubcommandExecutor for CompileSourceExecutor {
    fn execute(&self, _path: Option<String>) -> color_eyre::Result<()> {
        self.try_compile_and_execute()?;
        Ok(())
    }
}

impl CompileSourceExecutor {
    fn try_compile_and_execute(&self) -> color_eyre::Result<()> {
        let path_exe = std::env::current_exe()?
            .parent()
            .ok_or_else(|| {
                BridgerError::Subcommand("Can not get the binary path for bridger".to_string())
            })?
            .join("");
        tracing::trace!(target: "bridger", "The execute path is: {}", path_exe.display());

        let mut exists = false;
        for prefix in support_common::constants::ALLOW_BINARY_PREFIX {
            let mut path_bridge = path_exe.join("../../../bridges").join(&self.command);
            tracing::trace!(target: "bridger", "Try detect binary fo path: {}", path_bridge.display());
            let full_command = format!("{}{}", prefix, self.command);
            if !path_bridge.exists() {
                path_bridge = path_exe.join("../../../bridges").join(&full_command);
                if !path_bridge.exists() {
                    continue;
                }
            }
            if let Err(e) = self.try_compile_and_execute_with_command(path_bridge, full_command) {
                if let Some(BridgerError::Subcommand(msg)) = e.downcast_ref() {
                    tracing::error!(target: "bridger", "{}", msg);
                    continue;
                }
            }
            exists = true;
            break;
        }
        if !exists {
            return Err(BridgerError::UnsupportExternal(format!(
                "Not support this subcommand: {}",
                self.command
            ))
            .into());
        }
        Ok(())
    }

    fn try_compile_and_execute_with_command(
        &self,
        path_bridge: PathBuf,
        command: impl AsRef<str>,
    ) -> color_eyre::Result<()> {
        let command = command.as_ref();
        tracing::info!(
            target: "bridger",
            "Try compile {} in path: {}",
            &command.blue(),
            path_bridge.display()
        );
        let mut args = Vec::<String>::new();
        if let Some(toolchain) = &self.toolchain_version {
            args.push(format!("+{}", toolchain));
        }
        args.push("build".to_string());
        if self.channel == CompileChannel::Release {
            let name = format!("--{}", self.channel.name());
            args.push(name);
        }
        args.push("-p".to_string());
        args.push(command.to_string());
        let args = args.as_slice();

        let mut builder_cargo = ProcessBuilder::new("cargo");
        builder_cargo.args(args).cwd(&path_bridge);

        tracing::info!(
            target: "bridger",
            "Execute `{} {}` in path: {}",
            "cargo".green(),
            args.join(" ").green(),
            path_bridge.display()
        );
        if let Err(e) = builder_cargo.exec() {
            return Err(BridgerError::Process(
                "cargo".to_string(),
                args.join(" "),
                format!("{:?}", e),
            )
            .into());
        }

        // when compiled success, prepare execute this binary

        let base_path = path_bridge.join("target").join(self.channel.name());
        let platform_command = if cfg!(windows) {
            format!("{}.exe", &command)
        } else {
            command.to_string()
        };
        let path_binary = base_path.join(&platform_command);
        if !path_binary.exists() {
            return Err(BridgerError::Subcommand(format!(
                "The command {} not found in path: {}",
                &platform_command,
                base_path.display()
            ))
            .into());
        }

        external::provider::common::execute_binary(
            command.to_string(),
            path_binary,
            self.args.clone(),
            path_bridge,
        )
    }
}
