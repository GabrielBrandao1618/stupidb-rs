use pest::Parser;

#[derive(pest_derive::Parser)]
#[grammar = "query/grammar/stupid-query-lang.pest"]
pub struct StupidQueryLangParser;

pub fn parse(input: &str) {
    let pairs = StupidQueryLangParser::parse(Rule::stmt, input).unwrap();
    println!("{:#?}", pairs);
}
