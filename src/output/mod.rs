use std::io::Write;
use std::str::FromStr;

use crate::output::console::ConsoleOutput;
use crate::output::json::JsonOutput;
use crate::output::yaml::YamlOutput;
use anyhow::Result;
use serde::{Deserialize, Serialize};

pub mod console;
pub mod json;
pub mod yaml;

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub enum OutputFormat {
    Console,
    Yaml,
    Json,
}

pub trait OutputTrait {
    fn display<'a, T: Deserialize<'a> + Serialize>(
        &self,
        writer: impl Write,
        obj: &T,
        include_keys: Option<Vec<&str>>,
        exclude_keys: Option<Vec<&str>>,
    ) -> Result<()>;
}

impl FromStr for OutputFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "console" => Ok(OutputFormat::Console),
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            _ => Err(format!("{} not supported format", s)),
        }
    }
}

pub struct OutputFactory {
    pub output: OutputFormat,
}

impl OutputFactory {
    pub fn new(output: OutputFormat) -> Self {
        Self { output }
    }
}

impl OutputTrait for OutputFactory {
    fn display<'a, T: Deserialize<'a> + Serialize>(
        &self,
        writer: impl Write,
        obj: &T,
        include_keys: Option<Vec<&str>>,
        exclude_keys: Option<Vec<&str>>,
    ) -> Result<()> {
        match self.output {
            OutputFormat::Console => {
                ConsoleOutput::new().display(writer, obj, include_keys, exclude_keys)
            }

            OutputFormat::Yaml => {
                YamlOutput::new().display(writer, obj, include_keys, exclude_keys)
            }

            OutputFormat::Json => {
                JsonOutput::new().display(writer, obj, include_keys, exclude_keys)
            }
        }
    }
}
