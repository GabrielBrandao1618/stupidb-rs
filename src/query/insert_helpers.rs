use pest::iterators::Pair;
use super::{Rule,extract_many_from_pair};

pub fn extract_insert_params(insert_action: &Pair<Rule>) -> (String, u32) {
    let params = extract_many_from_pair(Rule::insertParam, insert_action.clone());
    let mut name = "".to_owned();
    let mut age: u32 = 0;
    for param in params {
        let (key, val) = extract_param_key_value(param);
        match key {
            "name" => name = val.to_owned(),
            "age" => {
                let parsed_age = val.parse::<u32>().unwrap();
                age = parsed_age;
            }
            _ => unreachable!(),
        }
    }
    return (name, age);
}
fn extract_param_key_value(param: Pair<Rule>) -> (&str, &str) {
    let key = param.clone().into_inner().nth(0).unwrap().as_str();
    let value = param
        .clone()
        .into_inner()
        .nth(1)
        .unwrap()
        .into_inner()
        .next()
        .unwrap()
        .as_str();
    return (key, value);
}
