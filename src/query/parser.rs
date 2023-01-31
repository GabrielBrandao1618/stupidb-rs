use std::collections::HashMap;

use pest::{iterators::Pair, iterators::Pairs, Parser};

use crate::model::Person;

#[derive(pest_derive::Parser)]
#[grammar = "query/grammar/stupid-query-lang.pest"]
pub struct StupidQueryLangParser;

pub fn parse(input: &str) {
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
                perform_action(action);
            }
            _ => unreachable!(),
        }
    }
}

pub fn perform_action(action: Pair<Rule>) {
    match action.as_rule() {
        Rule::select => {
            println!("Performing a select");
            let conditions = action.into_inner().next().unwrap().into_inner();

            let p1 = Person::new("Gabriel".to_owned(), 18);
            satisfies_where(conditions, &p1);
        }
        Rule::insert => println!("Performing a insert"),
        Rule::delete => println!("Performing a delete"),
        _ => unreachable!(),
    }
}

fn satisfies_where(conditions: Pairs<Rule>, person: &Person) -> bool {
    let mut result = false;
    for condition in conditions {
        match condition.as_rule() {
            Rule::comparision => {
                resolve_comparision(condition, person);
            }
            Rule::comparisionSegment => println!("Comparision segment"),
            _ => unreachable!(),
        }
    }
    return result;
}
fn resolve_comparision<V>(comparision: Pair<Rule>, base_value: V) -> bool {
    let comparision_vec: Vec<Pair<Rule>> = comparision.into_inner().collect();
    let operator = comparision_vec[1]
        .clone()
        .into_inner()
        .next()
        .unwrap()
        .as_rule();

    let attribute = comparision_vec[0].clone();
    
    let value = comparision_vec[2].clone().into_inner().next().unwrap();
    match value.as_rule() {
        Rule::int => {
            let parsed_value: u32 = value.as_str().parse::<u32>().unwrap();
            println!("It is int, {}", parsed_value);
            return parsed_value == base_value;
        }
        Rule::string => println!("It is string"),
        _ => unreachable!(),
    }
    println!("{:#?}", value);

    match operator {
        Rule::equals => {
            println!("Checking if is equals");
        }
        Rule::minor => {
            println!("Checking if is minor");
        }
        Rule::greater => {
            println!("Checking if is greater");
        }
        _ => unreachable!(),
    }
    return false;
}
