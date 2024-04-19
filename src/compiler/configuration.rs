use crate::common_configuration::CommonConfiguration;
use std::path::PathBuf;

#[derive(Debug)]
pub struct CompilerConfiguration {
    pub common: CommonConfiguration,
    //Those can be files or directories
    pub paths_to_compile: Vec<PathBuf>,
    pub output_folder: PathBuf,
}
