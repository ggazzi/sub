use std::process::exit;
use std::path::PathBuf;

use clap::{Command, ColorChoice, Arg, ArgGroup};
use clap::builder::Styles;

#[derive(Clone)]
pub enum Color {
    Auto,
    Always,
    Never,
}

#[derive(Clone)]
pub struct Config {
    pub name: String,
    pub color: Color,
    pub root: PathBuf,
    pub cache_directory: PathBuf,
}

impl Config {
    pub fn new(name: String, root: PathBuf, color: Color) -> Config {
        let xdg_dirs = match xdg::BaseDirectories::with_prefix(&name) {
            Ok(dir) => dir,
            Err(e) => {
                println!("Problem determining XDG base directory");
                println!("Original error: {}", e);
                exit(1);
            }
        };
        let cache_directory = match xdg_dirs.create_cache_directory("cache") {
            Ok(dir) => dir,
            Err(e) => {
                println!("Problem determining XDG cache directory");
                println!("Original error: {}", e);
                exit(1);
            }
        };

        Config {
            name,
            color,
            root,
            cache_directory,
        }
    }

    pub fn libexec_path(&self) -> PathBuf {
        let mut path = self.root.clone();
        path.push("libexec");
        return path;
    }

    pub fn base_command(&self, name: &str) -> Command {
        let color_choice = match self.color {
            Color::Auto => ColorChoice::Auto,
            Color::Always => ColorChoice::Always,
            Color::Never => ColorChoice::Never,
        };

        let styles = match self.color {
            Color::Auto => Styles::default(),
            Color::Always => Styles::default(),
            Color::Never => Styles::plain(),
        };

        Command::new(name.to_owned()).color(color_choice).styles(styles)
    }

    pub fn user_cli_command(&self, name: &str) -> Command {
        self.base_command(name).no_binary_name(true).disable_help_flag(true)
            .arg(Arg::new("usage").long("usage").num_args(0).help("Print usage"))
            .arg(Arg::new("help").short('h').long("help").num_args(0).help("Print help"))
            .arg(Arg::new("completions").long("completions").num_args(0).help("Print completions"))

            .arg(Arg::new("commands").long("commands").num_args(0).help("Print subcommands"))
            .arg(Arg::new("extension").long("extension").num_args(1).help("Filter subcommands by extension"))
            .group(ArgGroup::new("extension_group").args(["extension"]).requires("commands"))

            .group(ArgGroup::new("exclusion").args(["commands", "completions", "usage", "help"]).multiple(false).required(false))

            .arg(Arg::new("commands_with_args").trailing_var_arg(true).allow_hyphen_values(true).num_args(..))
    }
}
