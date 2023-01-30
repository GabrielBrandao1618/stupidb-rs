use pest::{Parser, iterators::Pair};

#[derive(pest_derive::Parser)]
#[grammar = "query/grammar/stupid-query-lang.pest"]
pub struct StupidQueryLangParser;

pub fn parse(input: &str) {
    let pairs = StupidQueryLangParser::parse(Rule::stmt, input).unwrap();
    for pair in pairs {
        match pair.as_rule() {
            Rule::EOI => break,
            Rule::stmt => {
                let action = pair.into_inner().next().unwrap().into_inner().next().unwrap();
                perform_action(action);
            },
            _ => unreachable!(),
        }
    }
}

pub fn perform_action(action: Pair<Rule>) {
    match action.as_rule() {
        Rule::select => {
            println!("Performing a select");
            let where_subtree = action.into_inner().next().unwrap();
            println!("{:#?}", where_subtree);
        },
        Rule::insert => println!("Performing a insert"),
        Rule::delete => println!("Performing a delete"),
        _ => unreachable!(), 
    }
}
