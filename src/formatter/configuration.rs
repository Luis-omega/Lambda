use crate::common_configuration::CommonConfiguration;
use std::path::PathBuf;

#[derive(Debug)]
pub enum FormatterOutput {
    ToFolder(PathBuf),
    ToStdout,
    InPace,
}

#[derive(Debug)]
pub struct FormatterConfiguration {
    pub common: CommonConfiguration,
    pub line_width: u16,
    pub indentation_length: u8,
    pub output_place: FormatterOutput,
}
