use crate::compiler::configuration::CompilerConfiguration;
use std::path::PathBuf;

use anyhow::{anyhow, Result};
//use itertools::{Either, Itertools};

#[derive(Debug)]
pub enum ParsedFile {}

pub fn report_error() {}

//pub fn load_and_parse_file(path: PathBuf) -> Result<ParsedFile> {
//    "Uninplemented".into()
//}
//
//pub fn split_from_errors<T>(
//    results: Vec<Result<T>>,
//) -> (Vec<T>, Vec<anyhow::Error>) {
//    results.into_iter().partition_map(|x| match x {
//        Err(x) => Either::Left(x),
//        Ok(x) => Either::Right(x),
//    })
//}
//
pub fn compile(config: CompilerConfiguration) {
    //config.paths_to_compile.into_iter().map(load_and_parse_file);
    dbg!(config);
    ()
}
