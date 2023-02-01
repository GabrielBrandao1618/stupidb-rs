use super::parser::Rule;

use crate::model::Person;
use pest::{iterators::Pair, iterators::Pairs};

pub fn extract_conditions_from_action(action: Pair<Rule>) -> Option<Pairs<Rule>> {
    match action.into_inner().next() {
        Some(inner) => {
            if inner.as_rule() != Rule::comparisionSegment && inner.as_rule() != Rule::comparision {
                return None;
            }
            return Some(inner.into_inner());
        },
        None => return None,
    }
}

pub fn extract_limit_from_action(action: &Pair<Rule>) -> u32 {
    let mut limit = 50;

    match action.clone().into_inner().next() {
        Some(lim) => {
            limit = lim
                .into_inner()
                .next()
                .unwrap()
                .as_str()
                .parse::<u32>()
                .unwrap();
        }
        None => (),
    };
    limit
}

pub fn satisfies_where(conditions: &mut Pairs<Rule>, person: &Person) -> bool {
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
            _ => return true,
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
                return compare_by_operator(&operator, base_value.age as u32, parsed_value);
            }
            Rule::string => println!("Invalid age argument: it must be a integral"),
            _ => unreachable!(),
        },
        "name" => match value.as_rule() {
            Rule::string => {
                let compare_value = base_value.name.to_owned();
                return compare_by_operator(&operator, compare_value, value.as_str().to_owned());
            }
            _ => unreachable!(),
        },
        "id" => match value.as_rule() {
            Rule::int => println!("Id must me uuid string"),
            Rule::string => {
                let compare_value = base_value.id.to_owned();
                return compare_by_operator(&operator, compare_value, value.as_str().to_owned());
            }
            _ => unreachable!(),
        },
        _ => println!("Unknown attribute"),
    }

    return false;
}

fn compare_by_operator<V: Ord>(operator: &Rule, a: V, b: V) -> bool {
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
