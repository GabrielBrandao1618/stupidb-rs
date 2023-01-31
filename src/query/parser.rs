use pest::{iterators::Pair, iterators::Pairs, Parser};

use crate::model::Person;
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

fn extract_conditions_from_action(action: Pair<Rule>) -> Pairs<Rule> {
    action.into_inner().next().unwrap().into_inner()
}

pub fn perform_action(action: Pair<Rule>) -> ActionResult {
    match action.as_rule() {
        Rule::select => {
            let conditions = extract_conditions_from_action(action);
            let query_result: Vec<Person> = select::list(50)
                .into_iter()
                .filter(|register| satisfies_where(conditions.clone(), &register))
                .collect();
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

fn satisfies_where(conditions: Pairs<Rule>, person: &Person) -> bool {
    let mut result = false;
    for condition in conditions {
        match condition.as_rule() {
            Rule::comparision => {
                result = resolve_comparision(condition, person);
            }
            Rule::comparisionSegment => {
                let condition_vec = condition.into_inner();
                // First element is the operator(and/or) and second element is the comparision
                // itself
                let condition_val = condition_vec.clone().nth(1).unwrap();
                let operator = condition_vec
                    .clone()
                    .nth(0)
                    .unwrap()
                    .into_inner()
                    .next()
                    .unwrap();
                match operator.as_rule() {
                    Rule::or => {
                        result = result || resolve_comparision(condition_val, person);
                    }
                    Rule::and => {
                        result = result && resolve_comparision(condition_val, person);
                    }
                    _ => println!("Unknown operator: {:#?}", operator.as_rule()),
                }
            }
            _ => unreachable!(),
        }
    }
    return result;
}
fn resolve_comparision(comparision: Pair<Rule>, base_value: &Person) -> bool {
    let comparision_inner = comparision.into_inner();
    let operator = comparision_inner
        .clone()
        .nth(1)
        .unwrap()
        .clone()
        .into_inner()
        .next()
        .unwrap()
        .as_rule();

    let value = comparision_inner
        .clone()
        .nth(2)
        .unwrap()
        .clone()
        .into_inner()
        .next()
        .unwrap();
    // Attribute is the person attribute. example: age or name
    let attribute = comparision_inner.clone().nth(0).unwrap().clone().as_str();
    match attribute {
        "age" => match value.as_rule() {
            Rule::int => {
                let parsed_value: u32 = value.as_str().parse::<u32>().unwrap();
                return compare_by_operator(operator, base_value.age as u32, parsed_value);
            }
            Rule::string => println!("Invalid age argument: it must be a integral"),
            _ => unreachable!(),
        },
        "name" => match value.as_rule() {
            Rule::string => {
                let compare_value = base_value.name.to_owned();
                return compare_by_operator(operator, compare_value, value.as_str().to_owned());
            }
            _ => unreachable!(),
        },
        "id" => match value.as_rule() {
            Rule::int => println!("Id must me uuid string"),
            Rule::string => {
                let compare_value = base_value.id.to_owned();
                return compare_by_operator(operator, compare_value, value.as_str().to_owned());
            }
            _ => unreachable!(),
        },
        _ => println!("Unknown attribute"),
    }

    return false;
}

fn compare_by_operator<V: Ord>(operator: Rule, a: V, b: V) -> bool {
    match operator {
        Rule::equals => {
            return a == b;
        }
        Rule::minor => {
            return a < b;
        }
        Rule::greater => {
            return a > b;
        }
        _ => unreachable!(),
    }
}
