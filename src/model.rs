pub type ParserInput = String; // TODO: This should be genric or &str at a minimum
pub type AstParserResult = Result<(ParserInput, AstNode), anyhow::Error>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AstNode {
    rule_name: String,
    value: AstNodeValue
}

impl AstNode {
    pub(crate) fn new<T: AsRef<str>>(name: T, value: AstNodeValue) -> Self {
        Self {
            rule_name: String::from(name.as_ref()),
            value
        }
    }

    pub(crate) fn new_literal<T: AsRef<str>>(name: T, value: T) -> Self {
        Self::new(name, AstNodeValue::Literal(value.as_ref().to_string()))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AstNodeValue {
    Literal(String)
}
