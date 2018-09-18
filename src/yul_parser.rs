use yul::*;

use pest::Parser;
use pest::iterators::Pair;

#[derive(Parser)]
#[grammar = "yul.pest"]
struct BlockParser;

use std::fs::File;
use std::io::prelude::*;

fn file_to_string(path: &str) -> String {
    let mut file = File::open(path).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    content
}

impl FunctionDefinition {
    fn from(pair: Pair<Rule>) -> FunctionDefinition {
        FunctionDefinition {
            name: Identifier {
                identifier: "name".to_string(),
                yultype: None
            },
            parameters: vec![],
            returns: vec![],
            block: Block { statements: vec![] }
        }
    }
}

impl Statement {
    fn from(pair: Pair<Rule>) -> Statement {
        for p in pair.into_inner() {
            match p.as_rule() {
                Rule::function_definition => Statement::FunctionDefinition(FunctionDefinition::from(p)),
                Rule::break_continue => Statement::Break,
                _ => panic!("")
            }
        }
    }
}

impl Block {
    fn from(pair: Pair<Rule>) -> Block {
        let mut statements: Vec<Statement> = vec![];
        for p in pair.into_inner() {
            match p.as_rule() {
                Rule::statement => {
                    //statements.push(Statement::from(p));
                }
                _ => unreachable!()
            }
        }
        Block { statements }
    }
}

pub fn parse_block(source: &str) -> Block {
    let mut pairs = BlockParser::parse(Rule::block, &source).unwrap();
    Block::from(pairs.next().unwrap())
}

#[cfg(test)]
mod tests {
use super::*;

    #[test]
    fn empty_block() {
        let source = file_to_string("examples/empty_block.yul");
        let block = parse_block(&source);
        assert_eq!(
            source,
            block.to_string()
        );
    }

}
