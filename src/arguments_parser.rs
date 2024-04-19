use anyhow::Result;

#[derive(Debug)]
pub struct CommonConfiguration {
    output_folder: String,
}

#[derive(Debug)]
pub struct CompilerConfiguration {
    common: CommonConfiguration,
}

#[derive(Debug)]
pub struct RunConfiguration {
    common: CommonConfiguration,
}

#[derive(Debug)]
pub struct DocumentationConfiguration {
    common: CommonConfiguration,
}

#[derive(Debug)]
pub struct FormatterConfiguration {
    common: CommonConfiguration,
    line_width: u16,
    indentation_length: u8,
    format_in_place: bool,
}

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
            output_folder: "output".to_string(),
        },
    }))
}
