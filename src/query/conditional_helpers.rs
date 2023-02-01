use super::parser::Rule;

use crate::model::Person;
use pest::{iterators::Pair, iterators::Pairs};

pub fn extract_conditions_from_action(action: Pair<Rule>) -> Option<Pairs<Rule>> {
    for stmt in action.into_inner() {
        if stmt.as_rule() == Rule::whereStmt {
            return Some(stmt.into_inner());
        }
    }
    None
}

pub fn extract_limit_from_action(action: &Pair<Rule>) -> u32 {
    for pair in action.clone().into_inner() {
        if pair.as_rule() == Rule::limitStmt {
            return pair.into_inner().next().unwrap().as_str().parse::<u32>().unwrap();
        }
    }
    return 50;
}

pub fn satisfies_where(conditions: Pairs<Rule>, person: &Person) -> bool {
    let mut result = false;
    for condition in conditions {
        match condition.as_rule() {
            Rule::comparision => {
                let comparision_result = resolve_comparision(condition, person);
                result = comparision_result;
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

                let comparision_result = resolve_comparision(condition_val, person);
                match operator.as_rule() {
                    Rule::or => {
                        result = result || comparision_result;
                    }
                    Rule::and => {
                        result = result && comparision_result;
                    }
                    _ => println!("Unknown operator: {:#?}", operator.as_rule()),
                }
            }
            _ => {
                println!("Unknown comparision operator {:#?}", condition.as_rule());
                return true;
            }
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
