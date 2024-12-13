use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct TestParser;

pub type Tree<'a> = Pair<'a, Rule>;

pub fn parse(source: &str) -> Result<Tree, pest::error::Error<Rule>> {
    let mut it = TestParser::parse(Rule::Sentence, source)?;
    let sentence = it.next().unwrap();
    assert_eq!(it.next(), None);
    assert_eq!(sentence.as_rule(), Rule::Sentence);
    let mut it = sentence.into_inner();
    let sequence = it.next().unwrap();
    assert_eq!(it.next().unwrap().as_rule(), Rule::EOI);
    assert_eq!(it.next(), None);
    assert_eq!(sequence.as_rule(), Rule::Sequence);
    Ok(sequence)
}

pub mod gen;
