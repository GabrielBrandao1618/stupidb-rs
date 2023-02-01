mod conditional_helpers;
mod insert_helpers;
pub mod parser;

use pest::iterators::Pair;

#[derive(pest_derive::Parser)]
#[grammar = "query/grammar/stupid-query-lang.pest"]
pub struct StupidQueryLangParser;

pub fn extract_from_pair(rule: Rule, pair: Pair<Rule>) -> Option<Pair<Rule>> {
    for item in pair.into_inner() {
        if item.as_rule() == rule {
            return Some(item);
        }
    }
    return None;
}
pub fn extract_many_from_pair(rule: Rule, pair: Pair<Rule>) -> Vec<Pair<Rule>> {
    let result: Vec<Pair<Rule>> = pair
        .into_inner()
        .filter(|row| {
            return row.as_rule() == rule;
        })
        .collect();
    return result;
}
