use pest::{iterators::Pair, iterators::Pairs, Parser};

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

            let (min_age, max_age, exact_name) = get_where_params(conditions);
        }
        Rule::insert => println!("Performing a insert"),
        Rule::delete => println!("Performing a delete"),
        _ => unreachable!(),
    }
}

fn get_where_params(conditions: Pairs<Rule>) -> (Option<u32>, Option<u32>, Option<String>) {
    for condition in conditions {
        match condition.as_rule() {
            Rule::condition => println!("Condition"),
            Rule::operator => println!("Operator"),
            _ => println!("Unknown"),
        }
    }
    return (None, None, None);
}
