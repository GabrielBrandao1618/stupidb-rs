#[derive(pest_derive::Parser)]
#[grammar = "query/grammar/stupid-query-lang.pest"]
pub struct StupidQueryLangParser;
