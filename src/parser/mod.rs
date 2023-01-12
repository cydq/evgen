use pest_derive::{Parser};

#[derive(Parser)]
#[grammar = "grammars/v1.pest"]
pub struct EV1Parser;