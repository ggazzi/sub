use std::path::PathBuf;
use std::process;

use crate::config::Config;
use crate::usage::{self, Usage};
use crate::parser;
use crate::error::{Error, Result};
use crate::commands::Command;

pub struct FileCommand<'a> {
    names: Vec<String>,
    path: PathBuf,
    usage: Usage,
    args: Vec<String>,
    config: &'a Config,
}

impl<'a> FileCommand<'a> {
    pub fn new(names: Vec<String>, path: PathBuf, args: Vec<String>, config: &'a Config) -> Self {
        let mut cmd = vec![config.name.to_owned()];
        cmd.extend(names.iter().map(|s| s.to_owned()));
        let cmd = cmd.join(" ");

        let usage = usage::extract_usage(config, &path, &cmd);

        return Self {
            names,
            path,
            usage,
            args,
            config,
        };
    }
}

impl<'a> Command for FileCommand<'a> {
    fn name(&self) -> &str {
        self.names.last().unwrap()
    }

    fn summary(&self) -> String {
        self.usage.summary()
    }

    fn usage(&self) -> Result<String> {
        self.usage.validate()?;

        Ok(self.usage.generate().to_string())
    }

    fn help(&self) -> Result<String> {
        self.usage.validate()?;

        self.usage.help()
    }

    fn subcommands(&self) -> Vec<Box<dyn Command + '_>> {
        let subcommands = Vec::new();
        return subcommands;
    }

    fn completions(&self) -> Result<i32> {
        if parser::provides_completions(&self.path) {
            let mut command = process::Command::new(&self.path);

            command.arg("--complete");
            command.env(format!("_{}_ROOT", self.config.name.to_uppercase()), &self.config.root);

            let status = command.status().unwrap();

            return match status.code() {
                Some(code) => Ok(code),
                None => Err(Error::SubCommandInterrupted),
            };
        }
        Ok(0)
    }

    fn invoke(&self) -> Result<i32> {
        self.usage.validate()?;

        if !self.path.exists() {
            return Err(Error::UnknownSubCommand(self.names.last().unwrap().to_owned()));
        }

        let mut command = process::Command::new(&self.path);

        command.args(&self.args);

        command.env(format!("_{}_ROOT", self.config.name.to_uppercase()), &self.config.root);
        command.env(format!("_{}_CACHE", self.config.name.to_uppercase()), &self.config.cache_directory);
        command.env(format!("_{}_ARGS", self.config.name.to_uppercase()), &self.usage.parse_into_kv(&self.args)?);

        let status = command.status().unwrap();

        match status.code() {
            Some(code) => Ok(code),
            None => Err(Error::SubCommandInterrupted),
        }
    }
}
