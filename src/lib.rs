use std::collections::{HashMap, VecDeque};

use abnf::rulelist;
use abnf::types::{Rule, Node, StringLiteral};
use nom::Parser;

type AstParser = dyn Parser<String, AstNode, anyhow::Error>;

pub struct AstNode {
    rule_name: String,
    value: AstNodeValue
}

pub enum AstNodeValue {
    Literal(String)
}

pub fn parse_abnf(abnf: String) -> anyhow::Result<Vec<Rule>> {
    Ok(rulelist(&abnf)?)
}

pub fn create_parser(mut rules: VecDeque<Rule>) -> anyhow::Result<Box<AstParser>> {
    if rules.is_empty() {

    }
    let mut parsers = HashMap::new();
    let mut final_parser = None;

    while let Some(rule) = rules.pop_front() {
        match create_parser_helper(&rule, &parsers)? {
            Some(parser) => { 
                final_parser = Some(String::from(rule.name()));
                parsers.insert(String::from(rule.name()), parser);
            },
            None => rules.push_back(rule)
        }
    }

    let final_parser = final_parser.ok_or(anyhow::anyhow!("Failed to store a key for the final parser!"))?;
    parsers.remove(&final_parser).ok_or(anyhow::anyhow!("Last inserted parser wasn't in the parser map!"))
}

fn create_parser_helper(rule: &Rule, parsers: &HashMap<String, Box<AstParser>>) -> anyhow::Result<Option<Box<AstParser>>> {
    match rule.node() {
        Node::String(string_literal) => Some(string_literal_parser_generator(string_literal)).transpose(),
        _ => Ok(None)
    }
}

fn string_literal_parser_generator(string_literal: &StringLiteral) -> anyhow::Result<Box<AstParser>> {
    anyhow::bail!("TODO: Implement")
}
