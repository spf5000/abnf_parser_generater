use abnf::rulelist;
use abnf::types::{Rule, Node, Kind};
use std::fs;
use std::vec::Vec;
use std::collections::{HashMap, HashSet};
use petgraph::graph::{DiGraph, NodeIndex};
use std::collections::hash_map::RandomState;
use std::boxed::Box;
use petgraph::algo::toposort;
use petgraph::Direction;
use crate::{Parser, BoxedParser};
use std::borrow::Borrow;

const SMITHY_IDL_PATH: &str = "smithy/configuration/smithy-idl.txt";

pub fn parse_smithy(models: Vec<String>) -> anyhow::Result<()> {
    let rule_set = get_smithy_rules()?;
    let (mut dag, rule_name_map) = get_dag_from_rules(&rule_set)?;
    let parser: BoxedParser<ParserOutput> = build_parser(dag, rule_name_map)?;
    Ok(())
}

enum ParserOutput {
    Value(String),
    Reference(Box<ParserOutput>),
    References(Vec<ParserOutput>)
}

fn build_parser<'a, T>(mut dag: DiGraph<&str, ()>, rule_name_map: HashMap<&str, &Rule>) -> anyhow::Result<T>
where T: Parser<'a, ParserOutput> {
    let mut parser_map = HashMap::new();
    let mut top_level_parser: Option<T> = None; // last parser will be the top level parser for the rules. Assuming rules are a tree with one root.
    // Builder parser, rule by rule in reverse dependency order, adding them to the parser map for use by dependent rules.
    for node_id in toposort(&dag, None).map_err(|_| anyhow::Error::msg("Failed to toposort the dependency graph!"))? {
        let rule_name = dag.node_weight(node_id).ok_or(anyhow::Error::msg("Couldn't get the rulename from the DAG after sorting"))?;
        if dag.edges_directed(node_id, Direction::Incoming).next().is_some() {
            return Err(anyhow::Error::msg(format!("Could not resolve dependency graph. After topological sorting, rule {} still had dependencies that hadn't been resolved!", rule_name)))
        }
        let rule = rule_name_map.get(rule_name).ok_or(anyhow::Error::msg(format!("Couldn't find {} from dag in supported rules", rule_name)))?;
        top_level_parser = Some(build_rule_parser(rule.node(), &mut dag, &rule_name_map, &mut parser_map)?);
    }
    top_level_parser.ok_or(anyhow::Error::msg("Failed to get the last parser!"))
}

fn build_rule_parser<'a, T>(node: &Node,
                     dag: &mut DiGraph<&str, ()>,
                     rule_name_map: &HashMap<&str, &Rule>,
                     parser_map: &mut HashMap<&str, Box<Parser<ParserOutput>>>) -> anyhow::Result<T>
where T: Parser<'a, ParserOutput> {
    // match rule.node() {
    //     abnf::Node::Alternatives
    // }

    Err(anyhow::Error::msg("KaBoom!"))

}

fn get_smithy_rules() -> anyhow::Result<Vec<Rule>> {
    let mut smithy_idl = std::env::current_dir()?;
    smithy_idl.push(SMITHY_IDL_PATH);

    let smithy_idl = fs::read_to_string(smithy_idl)?;
    let rules = match rulelist(&smithy_idl) {
        Ok(rules) => rules,
        Err(err) => { eprintln!("{}", err); return Err(anyhow::Error::new(err))}
    };
    println!("{:#?}", rules);
    Ok(rules)
}

fn get_dag_from_rules(rule_set: &Vec<Rule>) -> anyhow::Result<(DiGraph<&str, ()>, HashMap<&str, &Rule, RandomState>)> {
    let rule_name_map: HashMap<&str, &Rule> = rule_set.iter()
        .map(|rule| (rule.name(), rule))
        .collect();
    let mut dependency_graph= DiGraph::new();
    let mut rule_id_map = HashMap::new();

    for rule in rule_set {
        if rule.kind() == Kind::Basic {
            let rule_index = add_rule_to_graph(&rule, &mut dependency_graph, &mut rule_id_map);
            let ri = rule_index.or_else(|| rule_id_map.get(rule.name()).map(|id| id.to_owned()));
            let dependencies = get_rule_dependencies(&rule);
            for dependency in dependencies {
                let dependency_rule = rule_name_map.get(dependency.as_str())
                    .ok_or(anyhow::Error::msg(format!("{} depends on {} which isn't a defined rule!", rule.name(), dependency)))?;
                let dependency_index = add_rule_to_graph(dependency_rule, &mut dependency_graph, &mut rule_id_map);
                let di = dependency_index.or_else(|| rule_id_map.get(dependency_rule.name()).map(|id| id.to_owned()));
                dependency_graph.add_edge(di.unwrap(), ri.unwrap(), ());
            }
        }
    }

    Ok((dependency_graph, rule_name_map))
}

fn add_rule_to_graph<'a>(rule: &'a Rule, dependency_graph: &mut DiGraph<&'a str, ()>, rule_id_map: &mut HashMap<&'a str, NodeIndex>) -> Option<NodeIndex> {
    // Only add the rule if it hasn't already been added.
    if !rule_id_map.contains_key(rule.name()) {
        let index = dependency_graph.add_node(rule.name());
        rule_id_map.insert(rule.name(), index);
        Some(index)
    } else {
        None
    }
}

fn get_rule_dependencies(rule: &Rule) -> Vec<&String> {
    let mut dependencies = Vec::new();
    get_node_dependencies(rule.node(), &mut dependencies);
    dependencies
}

fn get_node_dependencies<'a>(node: &'a Node, dependencies: &mut Vec<&'a String>) {
    match node {
        Node::Alternatives(nodes) => get_nodes_dependencies(nodes, dependencies),
        Node::Concatenation(nodes) => get_nodes_dependencies(nodes, dependencies),
        Node::Repetition {repeat, node} => get_node_dependencies(node, dependencies),
        Node::Group(boxed_node) => get_node_dependencies(boxed_node, dependencies),
        Node::Optional(boxed_node) => get_node_dependencies(boxed_node, dependencies),
        Node::String(string) => (),
        Node::TerminalValues(values) => (),
        Node::Prose(prose) => (),
        Node::Rulename(rulename) => dependencies.push(rulename)
    }
}

fn get_nodes_dependencies<'a>(nodes: &'a Vec<Node>, dependencies: &mut Vec<&'a String>) {
    for node in nodes { get_node_dependencies(node, dependencies) }
}


#[cfg(test)]
mod tests {

    #[test]
    fn it_works() -> anyhow::Result<()>{
        crate::parse_smithy(vec![])?;
        Ok(())
    }
}
