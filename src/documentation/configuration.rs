use crate::common_configuration::CommonConfiguration;

use std::path::PathBuf;

#[derive(Debug)]
pub struct DocumentationConfiguration {
    pub common: CommonConfiguration,
    pub output_folder: PathBuf,
}
