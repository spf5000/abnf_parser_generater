use std::collections::{HashMap, VecDeque};
use std::sync::Arc;

use abnf::rulelist;
use abnf::types::{Rule, Node};

pub mod model;
mod parsers;

use model::*;

// type AstParser = Fn(String) -> Result<(String, AstNode), anyhow::Error>;

pub fn parse_abnf(abnf: String) -> anyhow::Result<Vec<Rule>> {
    Ok(rulelist(&abnf)?)
}

pub fn create_parser(mut rules: VecDeque<Rule>) -> anyhow::Result<Arc<dyn Fn(String) -> AstParserResult>> {
    if rules.is_empty() {

    }
    let mut parsers = HashMap::new();
    let mut final_parser = None;

    while let Some(rule) = rules.pop_front() {
        let rule_name = String::from(rule.name());
        match create_parser_helper(rule, &parsers) {
            CreateParserOutput::Success(parser) => {
                final_parser = Some(rule_name.clone());
                parsers.insert(rule_name, parser);
            },
            CreateParserOutput::Pending(rule) => {
                rules.push_back(rule)
            }, 
            CreateParserOutput::Failed(err) => {
                return Err(err)
            }
        }
    }

    let final_parser = final_parser.ok_or(anyhow::anyhow!("Failed to store a key for the final parser!"))?;
    let output: Arc<dyn Fn(String) -> AstParserResult> = parsers.remove(&final_parser).ok_or(anyhow::anyhow!("Last inserted parser wasn't in the parser map!"))?;
    Ok(output)
}

pub(crate) enum CreateParserOutput {
    Success(Arc<dyn Fn(ParserInput) -> AstParserResult>),
    Pending(Rule),
    Failed(anyhow::Error)
}

fn create_parser_helper<'a>(rule: Rule, parsers: &HashMap<String, Arc<dyn Fn(ParserInput) -> AstParserResult>>) -> CreateParserOutput {
    match rule.node() {
        Node::String(_) => {
            match parsers::string_literal_parser_generator(rule) {
                Ok(parser) => CreateParserOutput::Success(parser),
                Err(e) => CreateParserOutput::Failed(e)
            }
        },
        Node::Rulename(_) => parsers::rule_name_parser_generator(rule, parsers),
        _ => CreateParserOutput::Failed(anyhow::anyhow!("Unhandled Node Type: {}", rule.node()))
    }
}
