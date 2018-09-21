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

impl Identifier {
    fn from(pair: Pair<Rule>) -> String {
        pair.as_str().to_string()
    }
}

impl Type {
    fn from(pair: Pair<Rule>) -> Type {
        let current = pair.into_inner().next().unwrap();
        match current.as_rule() {
            Rule::identifier => Type::Custom(Identifier::from(current)),
            Rule::builtin_typename => match current.as_str() {
                "bool" => Type::Bool,
                "u8" => Type::Uint8,
                "u32" => Type::Uint32,
                "u64" => Type::Uint64,
                "u128" => Type::Uint128,
                "u256" => Type::Uint256,
                "s8" => Type::Int8,
                "s32" => Type::Int32,
                "s64" => Type::Int64,
                "s128" => Type::Int128,
                "s256" => Type::Int256,
                _ => unreachable!()
            },
            _ => unreachable!()
        }
    }
}

fn parse_typed_identifier(pair: Pair<Rule>) -> Identifier {
    let mut token_iter = pair.into_inner();
    let identifier_str = Identifier::from(token_iter.next().unwrap());
    let current = token_iter.next().unwrap();
    let yultype = match current.as_rule() {
        Rule::type_name => Some(Type::from(current)),
        _ => None
    };
    Identifier {
        identifier: identifier_str,
        yultype: yultype
    }
}

fn parse_typed_parameter_list(pair: Pair<Rule>) -> Vec<Identifier> {
    let mut identifiers: Vec<Identifier> = vec![];
    for p in pair.into_inner() {
        match p.as_rule() {
            Rule::typed_identifier => {
                identifiers.push(parse_typed_identifier(p));
            }
            _ => unreachable!()
        }
    }
    identifiers
}

impl FunctionDefinition {
    fn from(pair: Pair<Rule>) -> FunctionDefinition {
        let mut token_iter = pair.into_inner();
        let name = Identifier::from(token_iter.next().unwrap());

        let current = token_iter.next().unwrap();
        let (parameters, current) = match current.as_rule() {
            Rule::typed_parameter_list => (parse_typed_parameter_list(current), token_iter.next().unwrap()),
            _ => (vec![], current)
        };
        let (returns, current) = match current.as_rule() {
            Rule::typed_identifier_list => (parse_typed_parameter_list(current), token_iter.next().unwrap()),
            _ => (vec![], current)
        };
        let block = Block::from(current);

        FunctionDefinition {
            name: Identifier {
                identifier: name,
                yultype: None
            },
            parameters,
            returns,
            block
        }
    }
}

impl Statement {
    fn from(pair: Pair<Rule>) -> Statement {
        let mut token_iter = pair.into_inner();
        let p = token_iter.next().unwrap();
        match p.as_rule() {
            Rule::function_definition => Statement::FunctionDefinition(FunctionDefinition::from(p)),
            _ => unreachable!()
        }
    }
}

impl Block {
    fn from(pair: Pair<Rule>) -> Block {
        let mut statements: Vec<Statement> = vec![];
        for p in pair.into_inner() {
            match p.as_rule() {
                Rule::statement => {
                    statements.push(Statement::from(p));
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
    fn empty_function() {
        let source = file_to_string("examples/power_function_signature.yul");
        let block = parse_block(&source);
        assert_eq!(
            source,
            block.to_string()
        );
    }

    #[test]
    fn empty_block() {
        let source = file_to_string("examples/empty_block.yul");
        //let source = file_to_string("examples/power_function_signature.yul");
        let block = parse_block(&source);
        assert_eq!(
            source,
            block.to_string()
        );
    }

}
