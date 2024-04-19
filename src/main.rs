mod arguments_parser;
mod compiler;
mod documentation_generator;
mod formatter;
mod run;

use arguments_parser::ParsedConfiguration::*;

fn main() {
    match arguments_parser::get_raw_arguments() {
        Ok(raw_arguments) => match arguments_parser::parse_arguments(raw_arguments) {
            Ok(parsed_arguments) => match parsed_arguments {
                Compile(config) => compiler::compile(config),
                Run(config) => run::run(config),
                Documentation(config) => documentation_generator::generate_documentation(config),
                // format name causes a problem?
                Format(config) => formatter::format_them(config),
            },
            Err(x) => print!("Error happened!"),
        },
        Err(x) => print!("Error happened!"),
    }
}
