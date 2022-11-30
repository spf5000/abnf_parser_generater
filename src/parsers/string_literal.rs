use abnf::types::{Rule, Node};
use crate::model::*;

// pub(crate)fn string_literal_parser_generator(rule_name: &str, string_literal: StringLiteral) -> impl Fn(String) -> AstParserResult + '_ {
pub(crate)fn string_literal_parser_generator(rule: Rule) -> anyhow::Result<Box<dyn Fn(String) -> AstParserResult>> {
    if let Node::String(string_literal) = rule.node().clone() {
        let parser = move |input: ParserInput| {
            let rule_value = string_literal.value();
            if string_literal.is_case_sensitive() && input.as_str().starts_with(&rule_value) {
                Ok(parse(rule.name(), rule_value, input))
            } else if string_literal.is_case_sensitive() && input.to_lowercase().starts_with(&rule_value.to_lowercase()) {
                Ok(parse(rule.name(), rule_value, input))
            } else {
                anyhow::bail!("Input {} does not start with {} with case-sensative as {}", &input, &rule_value, string_literal.is_case_sensitive())
            }
        };
        Ok(Box::new(parser))
    } else {
        Err(anyhow::anyhow!("TODO: Handle errors"))
    }
}

fn parse(rule_name: &str, rule_value: &str, input: ParserInput) -> (String, AstNode) {
    let remaining = input.as_str()[rule_value.len()-1..].to_string();
    let value = AstNodeValue::Literal(rule_value[..input.len()].to_string());
    (remaining, AstNode::new(rule_name, value))
}
