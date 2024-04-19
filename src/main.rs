mod arguments_parser;
mod common_configuration;
mod compiler;
mod documentation;
mod formatter;
mod run;

use arguments_parser::ParsedConfiguration::*;
use compiler::compiler::compile;
use documentation::documentation::generate_documentation;
use formatter::formatter::format_them;
use run::run::run;

fn main() {
    match arguments_parser::get_raw_arguments() {
        Ok(raw_arguments) => {
            match arguments_parser::parse_arguments(raw_arguments) {
                Ok(parsed_arguments) => match parsed_arguments {
                    Compile(config) => compile(config),
                    Run(config) => run(config),
                    Documentation(config) => generate_documentation(config),
                    // format name causes a problem?
                    Format(config) => format_them(config),
                },
                Err(x) => {
                    print!("Error happened!");
                    print!("{}", x)
                }
            }
        }
        Err(x) => {
            print!("Error happened!");
            println!("{}", x)
        }
    }
}
