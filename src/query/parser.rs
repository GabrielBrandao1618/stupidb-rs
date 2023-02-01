use pest::{iterators::Pair, Parser};

use crate::model::Person;
use crate::query::conditional_helpers::{
    extract_conditions_from_action, extract_limit_from_action, satisfies_where,
};
use crate::storage::select;

#[derive(pest_derive::Parser)]
#[grammar = "query/grammar/stupid-query-lang.pest"]
pub struct StupidQueryLangParser;

pub struct ActionResult {
    pub rows: Vec<Person>,
    pub msg: String,
}

pub fn parse(input: &str) -> ActionResult {
    let pairs = StupidQueryLangParser::parse(Rule::stmt, input).unwrap();
    for pair in pairs {
        match pair.as_rule() {
            Rule::EOI => break,
            Rule::stmt => {
                let action = pair
                    .into_inner()
                    .next()
                    .unwrap()
                    .into_inner()
                    .next()
                    .unwrap();
                return perform_action(action);
            }
            _ => unreachable!(),
        }
    }
    return ActionResult {
        rows: vec![],
        msg: "Could not perform action".to_owned(),
    };
}

pub fn perform_action(action: Pair<Rule>) -> ActionResult {
    match action.as_rule() {
        Rule::select => {
            let conditions = extract_conditions_from_action(action.clone());
            let limit = extract_limit_from_action(&action);
            let mut query_result: Vec<Person> = select::list(limit as usize);
            match conditions {
                Some(unwraped) => {
                    query_result = query_result
                        .into_iter()
                        .filter(|row| satisfies_where(unwraped.clone(), &row))
                        .collect()
                }
                None => (),
            }
            return ActionResult {
                msg: "Selected all people within the filter".to_owned(),
                rows: query_result,
            };
        }
        Rule::insert => println!("Performing a insert"),
        Rule::delete => println!("Performing a delete"),
        _ => unreachable!(),
    }
    return ActionResult {
        rows: vec![],
        msg: "Could not perform action".to_owned(),
    };
}
