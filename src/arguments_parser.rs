use crate::common_configuration::CommonConfiguration;
use crate::compiler::configuration::CompilerConfiguration;
use crate::documentation::configuration::DocumentationConfiguration;
use crate::formatter::configuration::FormatterConfiguration;
use crate::run::configuration::RunConfiguration;
use std::path::PathBuf;

use anyhow::Result;

#[derive(Debug)]
pub enum ParsedConfiguration {
    Compile(CompilerConfiguration),
    Run(RunConfiguration),
    Documentation(DocumentationConfiguration),
    Format(FormatterConfiguration),
}

pub fn get_raw_arguments() -> Result<String> {
    Ok("hi he hi".to_string())
}

pub fn parse_arguments(console_input: String) -> Result<ParsedConfiguration> {
    Ok(ParsedConfiguration::Compile(CompilerConfiguration {
        common: CommonConfiguration {
            invocation_path: PathBuf::from("."),
        },
        paths_to_compile: vec![],
        output_folder: PathBuf::from("./compilated"),
    }))
}
