use super::{insert_helpers::extract_insert_params, Rule, StupidQueryLangParser};

use pest::{iterators::Pair, Parser};

use crate::model::Person;
use crate::query::conditional_helpers::{
    extract_conditions_from_action, extract_limit_from_action,
};
use crate::storage::mutate::remove_by_key;
use crate::storage::{insert, select};

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
            let query_result: Vec<Person> = select::list(Some(limit as usize), conditions);
            return ActionResult {
                msg: "Selected all people within the filter".to_owned(),
                rows: query_result,
            };
        }
        Rule::insert => {
            let (name, age) = extract_insert_params(action);
            insert::create_person(name, age as u16);
        }
        Rule::delete => {
            let conditions = extract_conditions_from_action(action);
            let targets: Vec<Person> = select::list(None, conditions);
            for target in targets {
                remove_by_key(&target.id);
            }
        }
        _ => unreachable!(),
    }
    return ActionResult {
        rows: vec![],
        msg: "Could not perform action".to_owned(),
    };
}
